import { z } from 'zod'
import Parser from 'rss-parser'
import { generateObject } from 'ai'
import { openai } from '@ai-sdk/openai'
import fs from 'fs/promises'
import path from 'path'
import 'dotenv/config'

const NewsItemSchema = z.object({
  title: z.string(),
  url: z.string(),
  summary: z.string().max(50, 'Summary must be under 50 chars').describe('Catchy, punchy Chinese one-liner to cure FOMO'),
  tag: z.enum(['AI', 'Dev', 'Crypto', 'Tech', 'Other']).describe('Category of the news'),
  source: z.string()
})

const NewsListSchema = z.object({
  items: z.array(NewsItemSchema)
})

type NewsItem = z.infer<typeof NewsItemSchema>

async function main() {
  console.log('🚀 Starting daily news aggregation...')

  const parser = new Parser()
  // Using a more stable tech feed (Hacker News) as the previous one had malformed XML
  const feedUrl = 'https://news.ycombinator.com/rss'

  try {
    const feed = await parser.parseURL(feedUrl)
    console.log(`📡 Fetched ${feed.items.length} items from ${feed.title}`)

    const latestItems = feed.items.slice(0, 5)

    const articlesContent = latestItems.map((item, index) => {
      return `Article ${index + 1}:
Title: ${item.title}
Link: ${item.link}
Snippet: ${item.contentSnippet || item.content || ''}
`
    }).join('\n---\n')

    console.log('🤖 Generating summaries with OpenAI...')

    // Check if API key is set/valid before calling
    const apiKey = process.env.OPENAI_API_KEY
    const isMock = !apiKey || apiKey.includes('your_openai_api_key_here')

    let items: NewsItem[] = []

    if (isMock) {
      console.warn('⚠️  No valid OPENAI_API_KEY found. Using MOCK data for demonstration.')
      items = latestItems.map((item, i) => ({
        title: item.title || 'Unknown Title',
        url: item.link || 'https://example.com',
        summary: `[MOCK] This is a simulated summary for article ${i + 1} because no API key was provided.`,
        tag: 'Tech' as const,
        source: feed.title || 'Hacker News'
      }))
    } else {
      const { object } = await generateObject({
        model: openai('gpt-4o-mini'),
        schema: NewsListSchema,
        system: 'You are a tech trend watcher. Summarize these articles into catchy, punchy Chinese one-liners to cure FOMO.',
        prompt: `Summarize the following articles:\n\n${articlesContent}`
      })
      items = object.items
    }

    const finalData = items.map(item => ({
      ...item,
      source: feed.title || 'Unknown Source'
    }))

    const outputPath = path.resolve(process.cwd(), 'public/data/latest.json')
    await fs.writeFile(outputPath, JSON.stringify(finalData, null, 2), 'utf-8')

    console.log(`✅ Successfully saved ${finalData.length} items to ${outputPath}`)
  } catch (error) {
    console.error('❌ Error in build-daily script:', error)
    process.exit(1)
  }
}

main()
