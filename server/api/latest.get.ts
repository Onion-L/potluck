import { serverSupabaseClient } from '#supabase/server'
import type { Database } from '../../app/types/database.types'

export default defineEventHandler(async (event) => {
  const client = await serverSupabaseClient<Database>(event)

  // Fetch top 50 articles sorted by date
  const { data, error } = await client
    .from('articles')
    .select('title, url, summary, tags, source, published_at')
    .order('published_at', { ascending: false })
    .limit(50)

  if (error) {
    console.error('DB Fetch Error:', error)
    throw createError({ statusCode: 500, message: 'Failed to fetch news' })
  }

  // Map to frontend interface
  return data.map(item => ({
    title: item.title,
    url: item.url,
    summary: item.summary || '',
    tag: item.tags?.[0] || 'Tech',
    source: item.source || 'Unknown',
    publishedAt: item.published_at
  }))
})
