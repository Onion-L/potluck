import { serverSupabaseClient } from '#supabase/server'
import type { Database } from '../../app/types/database.types'

const parseDateInput = (value: unknown): string | null => {
  if (typeof value !== 'string') return null

  const trimmed = value.trim()

  if (!/^\d{4}-\d{2}-\d{2}$/.test(trimmed)) return null

  return trimmed
}

const shiftUtcDate = (dateString: string, days: number): string => {
  const [year, month, day] = dateString.split('-').map(Number)
  const date = new Date(Date.UTC(year || 2000, (month || 1) - 1, day || 1))
  date.setUTCDate(date.getUTCDate() + days)
  return date.toISOString()
}

export default defineEventHandler(async (event) => {
  const query = getQuery(event)
  const startDate = parseDateInput(query.startDate)
  const endDate = parseDateInput(query.endDate)

  const client = await serverSupabaseClient<Database>(event)

  if (startDate || endDate) {
    const normalizedStart = startDate || endDate!
    const normalizedEnd = endDate || startDate!

    if (normalizedStart > normalizedEnd) {
      throw createError({
        statusCode: 400,
        message: 'Invalid date range'
      })
    }

    const publishedAfterOrAt = shiftUtcDate(normalizedStart, -1)
    const publishedBefore = shiftUtcDate(normalizedEnd, 0)

    const { data, error } = await client
      .from('articles')
      .select('title, url, summary, tags, source, published_at')
      .gte('published_at', publishedAfterOrAt)
      .lt('published_at', publishedBefore)
      .order('published_at', { ascending: false })
      .limit(1000)

    if (error) {
      console.error('Timeline Range Fetch Error:', error)
      throw createError({ statusCode: 500, message: 'Failed to fetch timeline' })
    }

    return {
      data: (data || []).map((item) => {
        const publishedDate = new Date(item.published_at)
        publishedDate.setUTCDate(publishedDate.getUTCDate() + 1)
        const briefingDate = publishedDate.toISOString().slice(0, 10)

        return {
          title: item.title,
          url: item.url,
          summary: item.summary || '',
          tag: item.tags?.[0] || 'Tech',
          source: item.source || 'Unknown',
          publishedAt: item.published_at,
          briefingDate
        }
      }),
      nextCursor: null,
      hasMore: false
    }
  }

  // Safe cursor parsing with validation
  const rawCursor = query.cursor ? String(query.cursor) : null
  const parsedCursor = rawCursor ? new Date(rawCursor) : new Date()
  const cursor = isNaN(parsedCursor.getTime()) ? new Date() : parsedCursor
  const limit = Math.min(Number(query.limit) || 20, 50)

  const { data, error } = await client
    .from('articles')
    .select('title, url, summary, tags, source, published_at')
    .lt('published_at', cursor.toISOString())
    .order('published_at', { ascending: false })
    .limit(limit + 1)

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
    data: resultData.map((item) => {
      // Briefing date is the day after publication (content is summarized the next day)
      const publishedDate = new Date(item.published_at)
      publishedDate.setUTCDate(publishedDate.getUTCDate() + 1)
      const briefingDate = publishedDate.toISOString().slice(0, 10)

      return {
        title: item.title,
        url: item.url,
        summary: item.summary || '',
        tag: item.tags?.[0] || 'Tech',
        source: item.source || 'Unknown',
        publishedAt: item.published_at,
        briefingDate
      }
    }),
    nextCursor,
    hasMore
  }
})
