import { serverSupabaseClient } from '#supabase/server'
import type { Database } from '../../app/types/database.types'

const MAX_LIMIT = 100
const DEFAULT_LIMIT = 50

export default defineEventHandler(async (event) => {
  const client = await serverSupabaseClient<Database>(event)

  // Parse pagination parameters
  const query = getQuery(event)
  const page = Math.max(1, parseInt(query.page as string) || 1)
  const limit = Math.min(MAX_LIMIT, Math.max(1, parseInt(query.limit as string) || DEFAULT_LIMIT))
  const offset = (page - 1) * limit

  // Calculate date range (UTC) for filtering: yesterday 00:00 to now
  const now = new Date()
  const yesterday = new Date(now)
  yesterday.setUTCDate(yesterday.getUTCDate() - 1)
  yesterday.setUTCHours(0, 0, 0, 0)

  const startDate = yesterday.toISOString()
  const endDate = now.toISOString()

  // Fetch articles from today/yesterday, sorted by date with pagination
  const { data, error, count } = await client
    .from('articles')
    .select('title, url, summary, tags, source, published_at', { count: 'exact' })
    .gte('published_at', startDate)
    .lte('published_at', endDate)
    .order('published_at', { ascending: false })
    .range(offset, offset + limit - 1)

  if (error) {
    console.error('[Latest] DB Fetch Error:', error)
    throw createError({ statusCode: 500, message: 'Failed to fetch news' })
  }

  // Set cache headers (5 minutes for public caching)
  setHeader(event, 'Cache-Control', 'public, max-age=300, s-maxage=300')

  const articles = data?.map(item => ({
    title: item.title,
    url: item.url,
    summary: item.summary || '',
    tag: item.tags?.[0] || 'Tech',
    source: item.source || 'Unknown',
    publishedAt: item.published_at
  })) || []

  return {
    data: articles,
    pagination: {
      page,
      limit,
      total: count || 0,
      totalPages: Math.ceil((count || 0) / limit)
    }
  }
})
