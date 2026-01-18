import Parser from 'rss-parser'

interface NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
}

interface CacheEntry {
  data: NewsItem[]
  timestamp: number
}

// Simple in-memory cache
let cache: CacheEntry | null = null
const CACHE_TTL_MS = 5 * 60 * 1000 // 5 minutes
const REQUEST_TIMEOUT_MS = 10000 // 10 seconds

function isCacheValid(): boolean {
  if (!cache) return false
  return Date.now() - cache.timestamp < CACHE_TTL_MS
}

export default defineEventHandler(async (_event) => {
  // Return cached data if valid
  if (isCacheValid() && cache) {
    return cache.data
  }

  const feedUrl = process.env.RSS_FEED_URL || 'https://news.smol.ai/rss.xml'

  // Create parser with timeout using AbortController
  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS)

  const parser = new Parser({
    timeout: REQUEST_TIMEOUT_MS,
    requestOptions: {
      signal: controller.signal
    }
  })

  try {
    const feed = await parser.parseURL(feedUrl)
    clearTimeout(timeoutId)

    const items = feed.items.slice(0, 20)

    const parsedItems: NewsItem[] = items.map((item) => {
      const date = item.isoDate || item.pubDate || new Date().toISOString()

      let summary = item.contentSnippet || item.content || ''
      if (summary.length > 200) {
        summary = summary.substring(0, 200).trim() + '...'
      }

      return {
        title: item.title || 'Untitled',
        url: item.link || '#',
        summary: summary || 'No summary available.',
        tag: 'Tech',
        source: feed.title || 'RSS Feed',
        publishedAt: date
      }
    })

    // Update cache
    cache = {
      data: parsedItems,
      timestamp: Date.now()
    }

    return parsedItems
  } catch (error) {
    clearTimeout(timeoutId)

    // Return stale cache if available when fetch fails
    if (cache) {
      console.warn('RSS fetch failed, returning stale cache:', error)
      return cache.data
    }

    console.error('RSS Fetch Error:', error)
    throw createError({
      statusCode: 500,
      statusMessage: 'Failed to fetch news feed'
    })
  }
})
