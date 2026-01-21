import { serverSupabaseClient } from '#supabase/server'
import type { Database } from '../../app/types/database.types'

export default defineEventHandler(async (event) => {
  const query = getQuery(event)

  // Safe cursor parsing with validation
  const rawCursor = query.cursor ? String(query.cursor) : null
  const parsedCursor = rawCursor ? new Date(rawCursor) : new Date()
  // If invalid date (NaN), fallback to current time
  const cursor = isNaN(parsedCursor.getTime()) ? new Date() : parsedCursor

  const limit = Math.min(Number(query.limit) || 20, 50)

  const client = await serverSupabaseClient<Database>(event)

  // Cursor pagination: Fetch items OLDER than cursor
  const { data, error } = await client
    .from('articles')
    .select('title, url, summary, tags, source, published_at')
    .lt('published_at', cursor.toISOString())
    .order('published_at', { ascending: false })
    .limit(limit + 1) // Fetch one extra to check if there are more

  if (error) {
    console.error('Timeline Fetch Error:', error)
    throw createError({ statusCode: 500, message: 'Failed to fetch timeline' })
  }

  const items = data || []
  const hasMore = items.length > limit
  const resultData = hasMore ? items.slice(0, -1) : items

  // Next cursor is the published_at of the last item
  const lastItem = resultData[resultData.length - 1]
  const nextCursor = (resultData.length > 0 && lastItem)
    ? lastItem.published_at
    : null

  return {
    data: resultData.map(item => ({
      title: item.title,
      url: item.url,
      summary: item.summary || '',
      tag: item.tags?.[0] || 'Tech',
      source: item.source || 'Unknown',
      publishedAt: item.published_at
    })),
    nextCursor,
    hasMore
  }
})
