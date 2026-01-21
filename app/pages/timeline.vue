<script setup lang="ts">
interface NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
}

interface TimelineResponse {
  data: NewsItem[]
  nextCursor: string | null
  hasMore: boolean
}

// State
const articles = ref<NewsItem[]>([])
const nextCursor = ref<string | null>(null)
const isLoading = ref(false)
const hasMore = ref(true)
const error = ref<Error | null>(null)
const loadTrigger = ref<HTMLElement>()

// Load function
const loadMore = async () => {
  if (isLoading.value || !hasMore.value) return

  isLoading.value = true
  error.value = null

  try {
    const params = nextCursor.value ? { cursor: nextCursor.value } : {}
    const res = await $fetch<TimelineResponse>('/api/timeline', { params })

    if (res.data.length) {
      articles.value.push(...res.data)
    }

    nextCursor.value = res.nextCursor
    hasMore.value = res.hasMore
  } catch (e) {
    console.error('Timeline load failed:', e)
    error.value = e as Error
  } finally {
    isLoading.value = false
  }
}

// Grouping logic (recomputed on append)
const groupedNews = computed(() => {
  if (!articles.value.length) return {}

  const groups: Record<string, NewsItem[]> = {}

  // Data is already sorted by API, but safety sort doesn't hurt
  const sorted = [...articles.value].sort((a, b) =>
    new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime()
  )

  sorted.forEach((item) => {
    const dateKey = item.publishedAt.slice(0, 10)
    if (!groups[dateKey]) groups[dateKey] = []
    groups[dateKey].push(item)
  })

  return groups
})

// Initial load & Infinite Scroll
onMounted(() => {
  // Initial load
  loadMore()

  // Intersection Observer
  const observer = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting && hasMore.value && !isLoading.value) {
        loadMore()
      }
    },
    { rootMargin: '200px' }
  )

  if (loadTrigger.value) observer.observe(loadTrigger.value!)

  onUnmounted(() => observer.disconnect())
})

const formatDateHeader = (dateString: string) => {
  const today = new Date()
  const yesterday = new Date()
  yesterday.setDate(yesterday.getDate() - 1)

  const tStr = today.toLocaleDateString('en-CA')
  const yStr = yesterday.toLocaleDateString('en-CA')

  if (dateString === tStr) return 'Today'
  if (dateString === yStr) return 'Yesterday'

  const parts = dateString.split('-').map(Number)
  const y = parts[0] ?? 2000
  const m = (parts[1] ?? 1) - 1
  const d = parts[2] ?? 1
  const date = new Date(Date.UTC(y, m, d))

  return date.toLocaleDateString('en-US', {
    month: 'long',
    day: 'numeric',
    year: 'numeric',
    timeZone: 'UTC'
  })
}

const formatTime = (isoString: string) => {
  const date = new Date(isoString)
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    timeZone: 'UTC'
  })
}

const currentDate = new Date().toLocaleDateString('en-US', {
  weekday: 'long',
  year: 'numeric',
  month: 'long',
  day: 'numeric'
})
</script>

<template>
  <div class="min-h-screen bg-stone-50 dark:bg-stone-950 font-sans selection:bg-stone-200 dark:selection:bg-stone-800 transition-colors duration-300 bg-noise">
    <UContainer class="py-12 md:py-20 relative z-10 max-w-4xl">
      <AppHeader />

      <!-- Error State -->
      <div
        v-if="error && articles.length === 0"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-red-100 dark:bg-red-900/30 mb-6">
          <UIcon
            name="i-lucide-alert-circle"
            class="w-8 h-8 text-red-500"
          />
        </div>
        <h2 class="text-2xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-4">
          Failed to Load Timeline
        </h2>
        <p class="text-stone-500 dark:text-stone-400 max-w-md mx-auto mb-8">
          {{ error.message || 'Unable to fetch timeline data.' }}
        </p>
        <UButton
          color="primary"
          variant="soft"
          @click="loadMore"
        >
          Retry
        </UButton>
      </div>

      <!-- Empty State -->
      <div
        v-else-if="!isLoading && articles.length === 0"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-stone-100 dark:bg-stone-900 mb-6">
          <UIcon
            name="i-lucide-archive"
            class="w-8 h-8 text-stone-400"
          />
        </div>
        <h2 class="text-3xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-4">
          No History Yet
        </h2>
        <UButton
          to="/"
          color="primary"
          variant="soft"
        >
          Back to Today
        </UButton>
      </div>

      <!-- Timeline Content -->
      <div
        v-else
        class="space-y-24"
      >
        <section
          v-for="(items, date) in groupedNews"
          :key="date"
          class="relative"
        >
          <!-- Date Header -->
          <div class="sticky top-6 z-30 mb-12 flex justify-start -ml-4 md:-ml-8">
            <h2 class="text-4xl md:text-5xl font-serif font-bold text-stone-900 dark:text-stone-100 bg-stone-50/95 dark:bg-stone-950/95 backdrop-blur-md px-6 py-2 rounded-r-full shadow-sm border-l-8 border-stone-900 dark:border-stone-100">
              {{ formatDateHeader(date) }}
            </h2>
          </div>

          <div class="space-y-16 pl-6 md:pl-10 border-l-2 border-stone-200 dark:border-stone-800 ml-4 md:ml-8 relative">
            <article
              v-for="(item, index) in items"
              :key="index"
              class="relative pl-8 md:pl-12 group"
            >
              <div class="absolute -left-[9px] top-2 w-4 h-4 rounded-full bg-stone-200 dark:bg-stone-800 ring-4 ring-stone-50 dark:ring-stone-950 group-hover:bg-stone-900 dark:group-hover:bg-stone-100 group-hover:scale-125 transition-all duration-300 z-10" />

              <a
                :href="item.url"
                target="_blank"
                rel="noopener noreferrer"
                class="block group-hover:-translate-y-1 transition-transform duration-300 p-4 -m-4 rounded-xl"
              >
                <div class="flex flex-col gap-3">
                  <div class="flex items-center gap-3 text-xs font-medium uppercase tracking-wider text-stone-400 dark:text-stone-500 mb-1">
                    <span class="text-stone-900 dark:text-stone-100 font-bold">{{ formatTime(item.publishedAt) }}</span>
                    <span>•</span>
                    <span class="bg-stone-100 dark:bg-stone-900 px-2 py-0.5 rounded text-stone-600 dark:text-stone-400">{{ item.tag }}</span>
                    <span class="ml-auto flex items-center gap-1 group-hover:text-stone-900 dark:group-hover:text-stone-100 transition-colors">
                      {{ item.source }}
                      <UIcon
                        name="i-lucide-arrow-up-right"
                        class="w-3 h-3"
                      />
                    </span>
                  </div>

                  <h3 class="text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-white leading-tight decoration-stone-900/30 dark:decoration-stone-100/30 underline-offset-4 decoration-2 group-hover:underline transition-all">
                    {{ item.title }}
                  </h3>

                  <div class="text-stone-600 dark:text-stone-400 leading-relaxed text-base md:text-lg max-w-2xl">
                    <MarkdownRenderer :content="item.summary" />
                  </div>
                </div>
              </a>
            </article>
          </div>
        </section>
      </div>

      <!-- Load Trigger / Spinner -->
      <div
        ref="loadTrigger"
        class="py-12 text-center"
      >
        <div
          v-if="isLoading"
          class="inline-block p-3 rounded-full bg-stone-100 dark:bg-stone-900 animate-pulse"
        >
          <UIcon
            name="i-lucide-loader-2"
            class="w-6 h-6 text-stone-400 animate-spin"
          />
        </div>
        <div
          v-else-if="!hasMore && articles.length > 0"
          class="text-stone-400 font-serif italic"
        >
          End of history
        </div>
      </div>

      <!-- Footer -->
      <footer class="mt-20 pt-12 border-t border-stone-200 dark:border-stone-800 text-center text-sm text-stone-400 dark:text-stone-600 font-serif italic pb-12">
        <p>Generated by Potluck • {{ currentDate }}</p>
      </footer>
    </UContainer>
  </div>
</template>
