import OpenAI from 'openai'

// Initialize DeepSeek Client
const getAIClient = () => {
  const config = useRuntimeConfig()
  return new OpenAI({
    baseURL: 'https://api.deepseek.com',
    apiKey: config.deepseekApiKey
  })
}

export const generateSummary = async (title: string, content: string) => {
  const client = getAIClient()

  const prompt = `
  Analyze this tech news article.
  Title: ${title}
  Content: ${content.slice(0, 1000)}...

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

    const content = response.choices[0]?.message?.content
    if (!content) throw new Error('No AI content')

    return JSON.parse(content)
  } catch (e) {
    console.error('AI Gen Failed:', e)
    return { summary: content.slice(0, 200), tags: ['Tech'] } // Fallback
  }
}
