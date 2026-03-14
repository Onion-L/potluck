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

interface ActiveFilter {
  mode: 'single' | 'range'
  start: string
  end: string
}

// SSR with lazy client navigation: data loads on server for direct visits,
// but shows loading state during client-side navigation
const { data: initialData, error: initialError, status } = await useFetch<TimelineResponse>('/api/timeline', {
  lazy: true
})

const route = useRoute()
const router = useRouter()

// State (hydrated from SSR or updated on client navigation)
const articles = ref<NewsItem[]>(initialData.value?.data || [])
const nextCursor = ref<string | null>(initialData.value?.nextCursor || null)
const hasMore = ref(initialData.value?.hasMore ?? true)
const isLoading = ref(false)
const error = ref<Error | null>(initialError.value as Error | null)
const loadTrigger = ref<HTMLElement>()
const resultsAnchor = ref<HTMLElement>()
const singleDate = ref(typeof route.query.date === 'string' ? route.query.date : '')
const rangeStart = ref(typeof route.query.start === 'string' ? route.query.start : '')
const rangeEnd = ref(typeof route.query.end === 'string' ? route.query.end : '')
const filterPending = ref(false)
const filterError = ref<Error | null>(null)
const filteredArticles = ref<NewsItem[]>([])
const activeFilter = ref<ActiveFilter | null>(null)

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
  if (isLoading.value || !hasMore.value || !nextCursor.value || activeFilter.value) return

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

const visibleArticles = computed(() => activeFilter.value ? filteredArticles.value : articles.value)

const filterSummary = computed(() => {
  if (!activeFilter.value) return ''

  if (activeFilter.value.start === activeFilter.value.end) {
    return formatDateHeader(activeFilter.value.start)
  }

  return `${formatDateHeader(activeFilter.value.start)} - ${formatDateHeader(activeFilter.value.end)}`
})

// Grouping logic (recomputed on append)
const groupedNews = computed(() => {
  if (!visibleArticles.value.length) return {}

  const groups: Record<string, NewsItem[]> = {}

  // Data is already sorted by API, but safety sort doesn't hurt
  const sorted = [...visibleArticles.value].sort((a, b) =>
    new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime()
  )

  sorted.forEach((item) => {
    const dateKey = item.briefingDate
    if (!groups[dateKey]) groups[dateKey] = []
    groups[dateKey].push(item)
  })

  return groups
})

const replaceQuery = async (query: Record<string, string | undefined>) => {
  await router.replace({
    query: {
      ...Object.fromEntries(
        Object.entries(route.query).filter(([key]) => !['date', 'start', 'end'].includes(key))
      ),
      ...query
    }
  })
}

const scrollToResults = () => {
  requestAnimationFrame(() => {
    resultsAnchor.value?.scrollIntoView({
      behavior: 'smooth',
      block: 'start'
    })
  })
}

const runFilter = async (start: string, end: string, mode: ActiveFilter['mode']) => {
  filterPending.value = true
  filterError.value = null

  try {
    const res = await $fetch<TimelineResponse>('/api/timeline', {
      params: {
        startDate: start,
        endDate: end
      }
    })

    activeFilter.value = { start, end, mode }
    filteredArticles.value = res.data

    await replaceQuery({
      date: mode === 'single' ? start : undefined,
      start: mode === 'range' ? start : undefined,
      end: mode === 'range' ? end : undefined
    })

    scrollToResults()
  } catch (e) {
    console.error('Timeline filter failed:', e)
    filterError.value = e as Error
  } finally {
    filterPending.value = false
  }
}

const applySingleDateFilter = async () => {
  if (!singleDate.value) {
    filterError.value = new Error('Pick a date to locate articles.')
    return
  }

  rangeStart.value = singleDate.value
  rangeEnd.value = singleDate.value

  await runFilter(singleDate.value, singleDate.value, 'single')
}

const applyRangeFilter = async () => {
  if (!rangeStart.value || !rangeEnd.value) {
    filterError.value = new Error('Pick both a start date and an end date.')
    return
  }

  if (rangeStart.value > rangeEnd.value) {
    filterError.value = new Error('The start date must be before the end date.')
    return
  }

  singleDate.value = rangeStart.value === rangeEnd.value ? rangeStart.value : ''

  await runFilter(rangeStart.value, rangeEnd.value, 'range')
}

const clearFilter = async () => {
  activeFilter.value = null
  filteredArticles.value = []
  filterError.value = null
  filterPending.value = false
  singleDate.value = ''
  rangeStart.value = ''
  rangeEnd.value = ''

  await replaceQuery({
    date: undefined,
    start: undefined,
    end: undefined
  })
}

// Infinite Scroll (initial data already loaded via SSR)
onMounted(() => {
  // Intersection Observer
  const observer = new IntersectionObserver(
    (entries) => {
      const entry = entries[0]
      if (entry?.isIntersecting && hasMore.value && !isLoading.value && !activeFilter.value) {
        loadMore()
      }
    },
    { rootMargin: '200px' }
  )

  if (loadTrigger.value) observer.observe(loadTrigger.value!)

  const routeDate = typeof route.query.date === 'string' ? route.query.date : ''
  const routeStart = typeof route.query.start === 'string' ? route.query.start : ''
  const routeEnd = typeof route.query.end === 'string' ? route.query.end : ''

  if (routeDate) {
    singleDate.value = routeDate
    rangeStart.value = routeDate
    rangeEnd.value = routeDate
    void runFilter(routeDate, routeDate, 'single')
  } else if (routeStart && routeEnd) {
    rangeStart.value = routeStart
    rangeEnd.value = routeEnd
    void runFilter(routeStart, routeEnd, 'range')
  }

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
        v-else-if="status === 'success' && articles.length === 0 && !singleDate && !rangeStart && !rangeEnd"
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
        <section class="rounded-3xl border border-stone-200/80 dark:border-stone-800 bg-white/80 dark:bg-stone-900/70 backdrop-blur-sm p-4 sm:p-6 md:p-8 shadow-sm space-y-6">
          <div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
            <div>
              <p class="text-xs font-mono uppercase tracking-[0.24em] text-stone-400 dark:text-stone-500 mb-2">
                Time Locator
              </p>
              <h2 class="text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-stone-100">
                选择时间，一键定位文章
              </h2>
              <p class="mt-2 text-sm md:text-base text-stone-500 dark:text-stone-400 max-w-2xl">
                可以按单天查看，也可以按时间段拉出该范围内的全部文章。筛选后会暂停无限滚动，方便直接浏览结果。
              </p>
            </div>

            <div
              v-if="activeFilter"
              class="rounded-2xl bg-stone-100 dark:bg-stone-800/80 px-4 py-3 text-sm text-stone-600 dark:text-stone-300"
            >
              <div class="font-medium text-stone-900 dark:text-stone-100">
                {{ filterSummary }}
              </div>
              <div class="mt-1">
                {{ visibleArticles.length }} articles
              </div>
            </div>
          </div>

          <div class="grid gap-4 lg:grid-cols-2">
            <div class="rounded-2xl border border-stone-200 dark:border-stone-800 p-4 sm:p-5 bg-stone-50/70 dark:bg-stone-950/40">
              <label
                for="timeline-single-date"
                class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-2"
              >
                单天定位
              </label>
              <input
                id="timeline-single-date"
                v-model="singleDate"
                type="date"
                class="w-full rounded-xl border border-stone-300 dark:border-stone-700 bg-white dark:bg-stone-950 px-4 py-3 text-stone-900 dark:text-stone-100 outline-none transition-colors focus:border-stone-500 dark:focus:border-stone-500"
              >
              <div class="mt-3 flex flex-wrap gap-3">
                <UButton
                  color="primary"
                  variant="soft"
                  :loading="filterPending"
                  @click="applySingleDateFilter"
                >
                  查看当天文章
                </UButton>
              </div>
            </div>

            <div class="rounded-2xl border border-stone-200 dark:border-stone-800 p-4 sm:p-5 bg-stone-50/70 dark:bg-stone-950/40">
              <p class="block text-sm font-medium text-stone-700 dark:text-stone-300 mb-2">
                时间段定位
              </p>
              <div class="grid gap-3 sm:grid-cols-2">
                <input
                  v-model="rangeStart"
                  type="date"
                  aria-label="Start date"
                  class="w-full rounded-xl border border-stone-300 dark:border-stone-700 bg-white dark:bg-stone-950 px-4 py-3 text-stone-900 dark:text-stone-100 outline-none transition-colors focus:border-stone-500 dark:focus:border-stone-500"
                >
                <input
                  v-model="rangeEnd"
                  type="date"
                  aria-label="End date"
                  class="w-full rounded-xl border border-stone-300 dark:border-stone-700 bg-white dark:bg-stone-950 px-4 py-3 text-stone-900 dark:text-stone-100 outline-none transition-colors focus:border-stone-500 dark:focus:border-stone-500"
                >
              </div>
              <div class="mt-3 flex flex-wrap gap-3">
                <UButton
                  color="primary"
                  variant="soft"
                  :loading="filterPending"
                  @click="applyRangeFilter"
                >
                  查看时间段文章
                </UButton>
                <UButton
                  v-if="activeFilter"
                  color="neutral"
                  variant="ghost"
                  @click="clearFilter"
                >
                  清除筛选
                </UButton>
              </div>
            </div>
          </div>

          <div
            v-if="filterError"
            class="rounded-2xl border border-red-200 dark:border-red-900/60 bg-red-50 dark:bg-red-950/20 px-4 py-3 text-sm text-red-700 dark:text-red-300"
          >
            {{ filterError.message || 'Unable to filter timeline.' }}
          </div>
        </section>

        <div ref="resultsAnchor" />

        <div
          v-if="filterPending"
          class="py-10 text-center"
        >
          <div class="inline-block p-3 rounded-full bg-stone-100 dark:bg-stone-900 animate-pulse">
            <UIcon
              name="i-lucide-loader-2"
              class="w-6 h-6 text-stone-400 animate-spin"
            />
          </div>
        </div>

        <div
          v-else-if="activeFilter && visibleArticles.length === 0"
          class="rounded-3xl border border-dashed border-stone-300 dark:border-stone-700 py-16 px-6 text-center"
        >
          <div class="inline-block p-4 rounded-full bg-stone-100 dark:bg-stone-900 mb-5">
            <UIcon
              name="i-lucide-calendar-search"
              class="w-8 h-8 text-stone-400"
            />
          </div>
          <h3 class="text-2xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-3">
            这个时间范围内没有文章
          </h3>
          <p class="text-stone-500 dark:text-stone-400 max-w-xl mx-auto">
            可以换一个日期，或者扩大时间范围再试一次。
          </p>
        </div>

        <template v-if="!filterPending">
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
        </template>
      </div>

      <!-- Load Trigger / Spinner -->
      <div
        v-if="!activeFilter"
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
