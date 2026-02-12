<script setup lang="ts">
interface NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
  briefingDate: string
}

interface TimelineResponse {
  data: NewsItem[]
  nextCursor: string | null
  hasMore: boolean
}

// SSR with lazy client navigation: data loads on server for direct visits,
// but shows loading state during client-side navigation
const { data: initialData, error: initialError, status } = await useFetch<TimelineResponse>('/api/timeline', {
  lazy: true
})

// State (hydrated from SSR or updated on client navigation)
const articles = ref<NewsItem[]>(initialData.value?.data || [])
const nextCursor = ref<string | null>(initialData.value?.nextCursor || null)
const hasMore = ref(initialData.value?.hasMore ?? true)
const isLoading = ref(false)
const error = ref<Error | null>(initialError.value as Error | null)
const loadTrigger = ref<HTMLElement>()

// Sync state when lazy fetch completes (for client-side navigation)
watch(initialData, (newData) => {
  if (newData && articles.value.length === 0) {
    articles.value = newData.data || []
    nextCursor.value = newData.nextCursor
    hasMore.value = newData.hasMore
  }
})

// Load more function (client-side only, for infinite scroll)
const loadMore = async () => {
  if (isLoading.value || !hasMore.value || !nextCursor.value) return

  isLoading.value = true

  try {
    const res = await $fetch<TimelineResponse>('/api/timeline', {
      params: { cursor: nextCursor.value }
    })

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
    const dateKey = item.briefingDate
    if (!groups[dateKey]) groups[dateKey] = []
    groups[dateKey].push(item)
  })

  return groups
})

// Infinite Scroll (initial data already loaded via SSR)
onMounted(() => {
  // Intersection Observer
  const observer = new IntersectionObserver(
    (entries) => {
      const entry = entries[0]
      if (entry?.isIntersecting && hasMore.value && !isLoading.value) {
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

      <!-- Initial Loading State -->
      <div
        v-if="status === 'pending' && articles.length === 0"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-stone-100 dark:bg-stone-900 mb-6 animate-pulse">
          <UIcon
            name="i-lucide-loader-2"
            class="w-8 h-8 text-stone-400 animate-spin"
          />
        </div>
        <p class="text-stone-500 dark:text-stone-400">
          Loading timeline...
        </p>
      </div>

      <!-- Error State -->
      <div
        v-else-if="error && articles.length === 0"
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
        v-else-if="status === 'success' && articles.length === 0"
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
          <div class="sticky top-4 md:top-6 z-30 mb-8 md:mb-12 flex justify-start -ml-4 md:-ml-8">
            <h2 class="text-2xl sm:text-3xl md:text-5xl font-serif font-bold text-stone-900 dark:text-stone-100 bg-stone-50/95 dark:bg-stone-950/95 backdrop-blur-md px-4 md:px-6 py-2 rounded-r-full shadow-sm border-l-4 md:border-l-8 border-stone-900 dark:border-stone-100">
              {{ formatDateHeader(date) }}
            </h2>
          </div>

          <div class="space-y-12 md:space-y-16 pl-4 sm:pl-6 md:pl-10 border-l-2 border-stone-200 dark:border-stone-800 ml-3 sm:ml-4 md:ml-8 relative">
            <article
              v-for="(item, index) in items"
              :key="index"
              class="relative pl-5 sm:pl-6 md:pl-12 group"
            >
              <div class="absolute -left-[9px] sm:-left-2.5 top-1.5 sm:top-2 w-3 h-3 sm:w-4 sm:h-4 rounded-full bg-stone-200 dark:bg-stone-800 ring-2 sm:ring-4 ring-stone-50 dark:ring-stone-950 group-hover:bg-stone-900 dark:group-hover:bg-stone-100 group-hover:scale-125 transition-all duration-300 z-10" />

              <a
                :href="item.url"
                target="_blank"
                rel="noopener noreferrer"
                class="block group-hover:-translate-y-1 transition-transform duration-300 p-3 sm:p-4 -m-3 sm:-m-4 rounded-xl min-h-[44px]"
              >
                <div class="flex flex-col gap-2 sm:gap-3">
                  <div class="flex flex-wrap items-center gap-2 sm:gap-3 text-xs font-medium uppercase tracking-wider text-stone-400 dark:text-stone-500 mb-1">
                    <span class="text-stone-900 dark:text-stone-100 font-bold">{{ formatTime(item.publishedAt) }}</span>
                    <span class="hidden sm:inline">•</span>
                    <span class="bg-stone-100 dark:bg-stone-900 px-2 py-0.5 rounded text-stone-600 dark:text-stone-400">{{ item.tag }}</span>
                    <span class="ml-auto flex items-center gap-1 group-hover:text-stone-900 dark:group-hover:text-stone-100 transition-colors">
                      {{ item.source }}
                      <UIcon
                        name="i-lucide-arrow-up-right"
                        class="w-3 h-3"
                      />
                    </span>
                  </div>

                  <h3 class="text-xl sm:text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-white leading-snug sm:leading-tight decoration-stone-900/30 dark:decoration-stone-100/30 underline-offset-4 decoration-2 group-hover:underline transition-all">
                    {{ item.title }}
                  </h3>

                  <div class="text-stone-600 dark:text-stone-400 leading-relaxed text-sm sm:text-base md:text-lg max-w-2xl mb-3">
                    <MarkdownRenderer :content="item.summary" />
                  </div>

                  <AISummaryButton
                    :title="item.title"
                    :url="item.url"
                    variant="ghost"
                    size="sm"
                  />
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
