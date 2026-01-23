import OpenAI from 'openai'
import { z } from 'zod'

// Schema for AI-generated summary
const AISummarySchema = z.object({
  title: z.string().max(100),
  summary: z.string().max(500),
  tags: z.array(z.string()).max(3).default(['Tech'])
})

type AISummary = z.infer<typeof AISummarySchema>

// Schema for importance evaluation (Gatekeeper Level 1)
const ImportanceEvaluationSchema = z.object({
  isHighValue: z.boolean(),
  reason: z.string().max(100)
})

type ImportanceEvaluation = z.infer<typeof ImportanceEvaluationSchema>

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
    "title": "Concise Chinese title, < 50 chars. Capture the key news point.",
    "summary": "Chinese summary in markdown, < 300 chars. Focus on value/impact.",
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
      title: title,
      summary: truncatedContent.slice(0, 200),
      tags: ['Tech']
    }
  }
}

/**
 * Level 1 Gatekeeper: Evaluate if an article is high-value for senior developers
 * Uses lightweight AI call (~100 tokens) to filter out low-value content
 */
export const evaluateArticleImportance = async (
  title: string,
  content: string
): Promise<ImportanceEvaluation> => {
  const client = getAIClient()
  const truncatedContent = content.slice(0, 500) // Shorter to save tokens

  const prompt = `
Evaluate if this tech article is HIGH VALUE for senior developers.

Title: ${title}
Content: ${truncatedContent}...

REJECT (isHighValue: false):
- Minor version updates (e.g., "v1.2.3 released")
- Weekly digests/newsletters
- Beginner tutorials ("How to install X")
- Marketing/promotional content

ACCEPT (isHighValue: true):
- Deep technical analysis
- Major releases (new frameworks, v2.0, breaking changes)
- Real-world architecture/migration case studies
- Expert insights and industry trends

Output JSON only:
{
  "isHighValue": true/false,
  "reason": "Brief reason in English, <50 chars"
}
`

  try {
    const response = await client.chat.completions.create({
      messages: [{ role: 'user', content: prompt }],
      model: 'deepseek-chat',
      response_format: { type: 'json_object' },
      max_tokens: 100 // Limit output length
    })

    const responseContent = response.choices[0]?.message?.content
    if (!responseContent) {
      // Default to accept if no response
      return { isHighValue: true, reason: 'No AI response, defaulting to accept' }
    }

    const parsed = JSON.parse(responseContent)
    return ImportanceEvaluationSchema.parse(parsed)
  } catch (e) {
    // Default to accept on error to avoid losing potentially valuable content
    console.error('[GATEKEEPER] Evaluation failed:', e instanceof Error ? e.message : e)
    return { isHighValue: true, reason: 'Evaluation error, defaulting to accept' }
  }
}
