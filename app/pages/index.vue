<script setup lang="ts">
interface NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
}

// Dynamic fetch from server API
const { data: newsData, status, error } = await useFetch<NewsItem[]>('/api/latest')

// Dynamically compute the latest date from the data (timezone-safe)
const latestDate = computed(() => {
  if (!newsData.value?.length) return ''
  const dates = newsData.value.map((item) => {
    // Parse ISO string and extract date part safely
    return item.publishedAt.slice(0, 10)
  })
  return [...new Set(dates)].sort().reverse()[0] || ''
})

const todaysNews = computed(() => {
  if (!newsData.value || !latestDate.value) return []

  return newsData.value
    .filter(item => item.publishedAt.startsWith(latestDate.value))
    .sort((a, b) => new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime())
})

const formatTime = (isoString: string) => {
  // Use UTC to avoid timezone shifts
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
      <!-- Header -->
      <AppHeader />

      <!-- Loading State -->
      <div
        v-if="status === 'pending'"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-stone-100 dark:bg-stone-900 mb-6 animate-pulse">
          <UIcon
            name="i-lucide-loader-2"
            class="w-8 h-8 text-stone-400 animate-spin"
          />
        </div>
        <p class="text-stone-500 dark:text-stone-400">
          Loading today's briefing...
        </p>
      </div>

      <!-- Error State -->
      <div
        v-else-if="error"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-red-100 dark:bg-red-900/30 mb-6">
          <UIcon
            name="i-lucide-alert-circle"
            class="w-8 h-8 text-red-500"
          />
        </div>
        <h2 class="text-2xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-4">
          Failed to Load News
        </h2>
        <p class="text-stone-500 dark:text-stone-400 max-w-md mx-auto mb-8">
          {{ error.message || 'Unable to fetch latest news. Please try again later.' }}
        </p>
        <UButton
          color="primary"
          variant="soft"
          @click="$router.go(0)"
        >
          Retry
        </UButton>
      </div>

      <!-- Empty State -->
      <div
        v-else-if="status === 'success' && todaysNews.length === 0"
        class="py-20 text-center"
      >
        <div class="inline-block p-4 rounded-full bg-stone-100 dark:bg-stone-900 mb-6">
          <UIcon
            name="i-heroicons-newspaper"
            class="w-8 h-8 text-stone-400"
          />
        </div>
        <h2 class="text-3xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-4">
          No News Today
        </h2>
        <p class="text-stone-500 dark:text-stone-400 max-w-md mx-auto mb-8">
          The press seems quiet today. Check back later or explore previous days.
        </p>
        <UButton
          to="/timeline"
          size="xl"
          color="primary"
          variant="soft"
        >
          Browse History
        </UButton>
      </div>

      <main v-else-if="status === 'success' && todaysNews.length > 0">
        <!-- Quick Headlines List -->
        <section class="mb-16 md:mb-20">
          <div class="mb-6 flex justify-end items-baseline">
            <div class="text-xs font-mono font-bold text-stone-400 uppercase">
              {{ todaysNews.length }} Updates
            </div>
          </div>

          <ul class="flex flex-col">
            <li
              v-for="(item, index) in todaysNews"
              :key="`headline-${index}`"
              class="group border-b border-stone-200 dark:border-stone-800/60 last:border-0 border-dashed"
            >
              <a
                :href="item.url"
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-baseline gap-3 py-3 px-4 -mx-4 rounded-lg transition-all duration-300 group-hover:translate-x-2"
              >
                <!-- Marker -->
                <span class="text-stone-900 dark:text-stone-100 font-bold text-xs shrink-0 select-none">
                  —
                </span>

                <!-- Title -->
                <span class="font-serif font-bold text-base md:text-lg text-stone-900 dark:text-stone-100 leading-tight group-hover:text-stone-600 dark:group-hover:text-stone-300 transition-colors flex-1">
                  {{ item.title }}
                </span>

                <!-- Time -->
                <span class="font-mono text-xs font-bold text-stone-400 uppercase shrink-0 tracking-tighter">
                  {{ formatTime(item.publishedAt) }}
                </span>
              </a>
            </li>
          </ul>
        </section>

        <!-- Stories List -->
        <div class="grid gap-12">
          <article
            v-for="(item, index) in todaysNews"
            :key="index"
            class="group border-b border-stone-100 dark:border-stone-900/50 pb-12 last:border-0"
          >
            <a
              :href="item.url"
              target="_blank"
              rel="noopener noreferrer"
              class="grid md:grid-cols-[1fr,3fr] gap-6 md:gap-12 items-baseline p-6 -m-6 rounded-2xl transition-all duration-300 group-hover:translate-x-1"
            >
              <div class="flex items-center gap-3 text-xs font-medium uppercase tracking-wider text-stone-400 dark:text-stone-500">
                <span class="text-stone-900 dark:text-stone-100">{{ formatTime(item.publishedAt) }}</span>
                <span class="text-stone-300 dark:text-stone-700">/</span>
                <span>{{ item.tag }}</span>
              </div>

              <div>
                <h3 class="text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-stone-100 leading-tight mb-3 decoration-stone-900/30 dark:decoration-stone-100/30 underline-offset-4 decoration-2 group-hover:underline transition-all">
                  {{ item.title }}
                </h3>
                <div class="text-stone-600 dark:text-stone-400 leading-relaxed text-base md:text-lg mb-2">
                  <MarkdownRenderer
                    :content="item.summary"
                  />
                </div>
                <div class="text-xs text-stone-400 font-medium flex items-center gap-1 group-hover:text-stone-900 dark:group-hover:text-stone-100 transition-colors">
                  {{ item.source }} <UIcon
                    name="i-lucide-arrow-up-right"
                    class="w-3 h-3 transition-transform duration-300 group-hover:translate-x-0.5 group-hover:-translate-y-0.5"
                  />
                </div>
              </div>
            </a>
          </article>
        </div>

        <div class="mt-20 text-center">
          <NuxtLink
            to="/timeline"
            class="inline-flex items-center gap-2 text-lg font-serif italic text-stone-500 hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-200 transition-colors"
          >
            View Full Timeline <UIcon
              name="i-lucide-arrow-right"
              class="w-5 h-5"
            />
          </NuxtLink>
        </div>
      </main>

      <!-- Footer -->
      <footer class="mt-32 pt-12 border-t border-stone-200 dark:border-stone-800 text-center text-sm text-stone-400 dark:text-stone-600 font-serif italic pb-12">
        <p>Generated by Potluck • {{ currentDate }}</p>
      </footer>
    </UContainer>
  </div>
</template>
