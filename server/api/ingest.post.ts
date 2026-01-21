import { timingSafeEqual } from 'node:crypto'
import Parser from 'rss-parser'
import { serverSupabaseServiceRole } from '#supabase/server'
import { generateSummary } from '../utils/ingest'
import type { Database } from '../../app/types/database.types'

// Simple in-memory rate limiter
const rateLimitStore = new Map<string, { count: number, resetAt: number }>()
const RATE_LIMIT_WINDOW_MS = 60 * 1000 // 1 minute
const RATE_LIMIT_MAX_REQUESTS = 5

// RSS parser timeout (10 seconds)
const RSS_FETCH_TIMEOUT_MS = 10000

// Allowed URL schemes for RSS feeds (SSRF protection)
const ALLOWED_URL_SCHEMES = ['http:', 'https:']

/**
 * Constant-time comparison to prevent timing attacks
 */
function secureCompare(input: string, expected: string): boolean {
  if (!input || !expected) return false
  if (input.length !== expected.length) return false
  try {
    return timingSafeEqual(Buffer.from(input), Buffer.from(expected))
  } catch {
    return false
  }
}

/**
 * Check rate limit for a given key
 */
function checkRateLimit(key: string): boolean {
  const now = Date.now()
  const entry = rateLimitStore.get(key)

  if (!entry || now > entry.resetAt) {
    rateLimitStore.set(key, { count: 1, resetAt: now + RATE_LIMIT_WINDOW_MS })
    return true
  }

  if (entry.count >= RATE_LIMIT_MAX_REQUESTS) {
    return false
  }

  entry.count++
  return true
}

/**
 * Validate URL for SSRF protection
 */
function isValidFeedUrl(urlString: string): boolean {
  try {
    const url = new URL(urlString)

    // Only allow http/https
    if (!ALLOWED_URL_SCHEMES.includes(url.protocol)) {
      return false
    }

    // Block localhost, private IPs, and internal domains
    const hostname = url.hostname.toLowerCase()
    if (
      hostname === 'localhost'
      || hostname === '127.0.0.1'
      || hostname === '0.0.0.0'
      || hostname.startsWith('192.168.')
      || hostname.startsWith('10.')
      || hostname.startsWith('172.')
      || hostname.endsWith('.local')
      || hostname.endsWith('.internal')
    ) {
      return false
    }

    return true
  } catch {
    return false
  }
}

/**
 * Fetch RSS with timeout
 */
async function fetchRSSWithTimeout(parser: Parser, url: string): Promise<Parser.Output<Record<string, unknown>>> {
  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), RSS_FETCH_TIMEOUT_MS)

  try {
    // rss-parser doesn't support AbortSignal directly, so we use Promise.race
    const result = await Promise.race([
      parser.parseURL(url),
      new Promise<never>((_, reject) => {
        controller.signal.addEventListener('abort', () => {
          reject(new Error(`RSS fetch timeout after ${RSS_FETCH_TIMEOUT_MS}ms`))
        })
      })
    ])
    return result
  } finally {
    clearTimeout(timeoutId)
  }
}

export default defineEventHandler(async (event) => {
  // Rate limiting
  const clientIP = getRequestIP(event, { xForwardedFor: true }) || 'unknown'
  if (!checkRateLimit(clientIP)) {
    throw createError({
      statusCode: 429,
      statusMessage: 'Too Many Requests: Rate limit exceeded'
    })
  }

  // Auth check
  const config = useRuntimeConfig()
  const authHeader = getHeader(event, 'authorization')
  const token = authHeader?.replace('Bearer ', '')

  if (!config.ingestApiKey) {
    console.warn('INGEST_API_KEY not configured - endpoint is unprotected')
  } else if (!secureCompare(token || '', config.ingestApiKey)) {
    console.warn(`[INGEST] Auth failed | IP: ${clientIP} | Time: ${new Date().toISOString()}`)

    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized: Invalid or missing API key'
    })
  }

  // Use Service Role to bypass RLS for ingestion
  const client = serverSupabaseServiceRole<Database>(event)
  const parser = new Parser()

  // 1. Get Feeds
  const { data: feeds } = await client.from('feeds').select('*').eq('is_active', true)
  if (!feeds?.length) return { message: 'No active feeds' }

  const stats = {
    processed: 0,
    added: 0,
    skipped: 0,
    errors: 0,
    errorDetails: [] as string[]
  }

  for (const feed of feeds) {
    try {
      // SSRF protection: validate feed URL
      if (!isValidFeedUrl(feed.url)) {
        console.warn(`[INGEST] Skipping invalid/unsafe URL: ${feed.url}`)
        stats.errors++
        stats.errorDetails.push(`${feed.name}: Invalid or unsafe URL`)
        continue
      }

      // 2. Parse RSS with timeout
      const parsed = await fetchRSSWithTimeout(parser, feed.url)

      // 3. Get items to process (limit to 5 most recent)
      const itemsToProcess = parsed.items.slice(0, 5).filter(item => item.link && item.title)
      if (itemsToProcess.length === 0) {
        stats.processed++
        continue
      }

      // 4. Batch check for existing articles (N+1 optimization)
      const urls = itemsToProcess.map(item => item.link as string)
      const { data: existingArticles } = await client
        .from('articles')
        .select('url')
        .in('url', urls)
      const existingUrls = new Set(existingArticles?.map(a => a.url) || [])

      // 5. Process new items only
      for (const item of itemsToProcess) {
        if (existingUrls.has(item.link!)) {
          stats.skipped++
          continue
        }

        // AI Generation
        const aiData = await generateSummary(item.title!, item.contentSnippet || item.content || '')

        // Insert
        const { error: insertError } = await client.from('articles').insert({
          feed_id: feed.id,
          title: aiData.title,
          url: item.link!,
          summary: aiData.summary,
          tags: aiData.tags,
          source: feed.name,
          published_at: item.isoDate || new Date().toISOString()
        })

        if (insertError) {
          throw new Error(`Insert failed: ${insertError.message}`)
        }

        stats.added++
      }
      stats.processed++
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : 'Unknown error'
      console.error(`[INGEST] Feed Error [${feed.name}]:`, errorMsg)
      stats.errors++
      stats.errorDetails.push(`${feed.name}: ${errorMsg}`)
    }
  }
  return stats
})
