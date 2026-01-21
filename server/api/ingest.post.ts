import Parser from 'rss-parser'
import { serverSupabaseClient } from '#supabase/server'
import { generateSummary } from '../utils/ingest'
import type { Database } from '../../app/types/database.types'

export default defineEventHandler(async (event) => {
  // Auth check
  const config = useRuntimeConfig()
  const authHeader = getHeader(event, 'authorization')
  const token = authHeader?.replace('Bearer ', '')

  if (!config.ingestApiKey) {
    console.warn('⚠️ INGEST_API_KEY not configured - endpoint is unprotected')
  } else if (token !== config.ingestApiKey) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Unauthorized: Invalid or missing API key'
    })
  }

  const client = await serverSupabaseClient<Database>(event)
  const parser = new Parser()

  // 1. Get Feeds
  const { data: feeds } = await client.from('feeds').select('*').eq('is_active', true)
  if (!feeds?.length) return { message: 'No active feeds' }

  const stats = { processed: 0, added: 0, errors: 0 }

  for (const feed of feeds) {
    try {
      console.log(feed.url)
      // 2. Parse RSS
      const parsed = await parser.parseURL(feed.url)

      // 3. Process Items (Limit 5 most recent to save tokens/time)
      for (const item of parsed.items.slice(0, 5)) {
        if (!item.link || !item.title) continue

        // Check deduplication
        const { data: existing } = await client
          .from('articles')
          .select('id')
          .eq('url', item.link)
          .single()

        if (existing) continue

        // 4. AI Generation
        const aiData = await generateSummary(item.title, item.contentSnippet || item.content || '')

        // 5. Insert
        await client.from('articles').insert({
          feed_id: feed.id,
          title: item.title,
          url: item.link,
          summary: aiData.summary,
          tags: aiData.tags,
          source: feed.name,
          published_at: item.isoDate || new Date().toISOString()
        })

        stats.added++
      }
      stats.processed++
    } catch (e) {
      console.error(`Feed Error ${feed.name}:`, e)
      stats.errors++
    }
  }

  return stats
})
