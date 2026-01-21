import OpenAI from 'openai'
import { z } from 'zod'

// Schema for AI-generated summary
const AISummarySchema = z.object({
  summary: z.string().max(500),
  tags: z.array(z.string()).max(3).default(['Tech'])
})

type AISummary = z.infer<typeof AISummarySchema>

// Initialize DeepSeek Client
const getAIClient = () => {
  const config = useRuntimeConfig()
  return new OpenAI({
    baseURL: 'https://api.deepseek.com',
    apiKey: config.deepseekApiKey
  })
}

export const generateSummary = async (title: string, content: string): Promise<AISummary> => {
  const client = getAIClient()
  const truncatedContent = content.slice(0, 1000)

  const prompt = `
  Analyze this tech news article.
  Title: ${title}
  Content: ${truncatedContent}...

  Output valid JSON only:
  {
    "summary": "Chinese summary in markdown, < 200 chars. Focus on value/impact.",
    "tags": ["Tag1", "Tag2"] (Max 2 tags, English, e.g. "AI", "Rust", "Vue")
  }
  `

  try {
    const response = await client.chat.completions.create({
      messages: [{ role: 'user', content: prompt }],
      model: 'deepseek-chat',
      response_format: { type: 'json_object' }
    })

    const responseContent = response.choices[0]?.message?.content
    if (!responseContent) {
      throw new Error('No AI content in response')
    }

    // Parse and validate with Zod
    const parsed = JSON.parse(responseContent)
    const validated = AISummarySchema.parse(parsed)

    return validated
  } catch (e) {
    // Log error details for debugging
    if (e instanceof z.ZodError) {
      console.error('[AI] Schema validation failed:', e.issues)
    } else if (e instanceof SyntaxError) {
      console.error('[AI] JSON parse failed:', e.message)
    } else {
      console.error('[AI] Generation failed:', e instanceof Error ? e.message : e)
    }

    // Return fallback with truncated content as summary
    return {
      summary: truncatedContent.slice(0, 200),
      tags: ['Tech']
    }
  }
}
