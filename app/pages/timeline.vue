<script setup lang="ts">
import newsData from '~/assets/data/latest.json'

interface NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
}

const news = ref(newsData)

const groupedNews = computed(() => {
  if (!news.value) return {}

  const groups: Record<string, NewsItem[]> = {}

  // Sort items by date descending first
  const sortedNews = [...news.value].sort((a, b) =>
    new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime()
  )

  sortedNews.forEach((item) => {
    const date = new Date(item.publishedAt)
    const dateKey = date.toISOString().split('T')[0] // YYYY-MM-DD

    if (!groups[dateKey]) {
      groups[dateKey] = []
    }
    groups[dateKey].push(item)
  })

  return groups
})

const formatDateHeader = (dateString: string) => {
  const date = new Date(dateString)
  // Fix timezone issue by treating the YYYY-MM-DD as local or UTC consistently
  // Actually, creating Date from YYYY-MM-DD assumes UTC usually, but let's be safe
  // We'll use the date object created from the key

  const today = new Date()
  const yesterday = new Date()
  yesterday.setDate(yesterday.getDate() - 1)

  // Normalize to YYYY-MM-DD for comparison
  const dStr = date.toISOString().split('T')[0]
  const tStr = today.toISOString().split('T')[0]
  const yStr = yesterday.toISOString().split('T')[0]

  if (dStr === tStr) {
    return 'Today'
  } else if (dStr === yStr) {
    return 'Yesterday'
  } else {
    return date.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })
  }
}

const formatTime = (isoString: string) => {
  return new Date(isoString).toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
}

const currentDate = new Date().toLocaleDateString('en-US', {
  weekday: 'long',
  year: 'numeric',
  month: 'long',
  day: 'numeric'
})
</script>

<template>
  <div class="min-h-screen bg-stone-50 dark:bg-stone-950 font-sans selection:bg-orange-100 dark:selection:bg-orange-900/30 transition-colors duration-300 bg-noise">
    <UContainer class="py-12 md:py-20 relative z-10 max-w-4xl">
      <!-- Header -->
      <AppHeader />

      <!-- Timeline Feed -->
      <div class="space-y-24">
        <section
          v-for="(items, date) in groupedNews"
          :key="date"
          class="relative"
        >
          <!-- Date Header -->
          <div class="sticky top-6 z-30 mb-12 flex justify-start -ml-4 md:-ml-8">
            <h2 class="text-4xl md:text-5xl font-serif font-bold text-stone-900 dark:text-stone-100 bg-stone-50/95 dark:bg-stone-950/95 backdrop-blur-md px-6 py-2 rounded-r-full shadow-sm border-l-8 border-orange-500">
              {{ formatDateHeader(date) }}
            </h2>
          </div>

          <div class="space-y-16 pl-6 md:pl-10 border-l-2 border-stone-200 dark:border-stone-800 ml-4 md:ml-8 relative">
            <article
              v-for="(item, index) in items"
              :key="index"
              class="relative pl-8 md:pl-12 group"
            >
              <!-- Timeline Node -->
              <div class="absolute -left-[9px] top-2 w-4 h-4 rounded-full bg-stone-200 dark:bg-stone-800 ring-4 ring-stone-50 dark:ring-stone-950 group-hover:bg-orange-500 group-hover:scale-110 transition-all duration-300 z-10" />

              <!-- Content -->
              <a
                :href="item.url"
                target="_blank"
                rel="noopener noreferrer"
                class="block group-hover:-translate-y-1 transition-transform duration-300"
              >
                <div class="flex flex-col gap-3">
                  <div class="flex items-center gap-3 text-xs font-medium uppercase tracking-wider text-stone-400 dark:text-stone-500 mb-1">
                    <span class="text-orange-600 dark:text-orange-400 font-bold">{{ formatTime(item.publishedAt) }}</span>
                    <span>•</span>
                    <span class="bg-stone-100 dark:bg-stone-900 px-2 py-0.5 rounded text-stone-600 dark:text-stone-400">{{ item.tag }}</span>
                    <span class="ml-auto flex items-center gap-1 group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
                      {{ item.source }}
                      <UIcon
                        name="i-lucide-arrow-up-right"
                        class="w-3 h-3"
                      />
                    </span>
                  </div>

                  <h3 class="text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-white leading-tight group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
                    {{ item.title }}
                  </h3>

                  <p class="text-stone-600 dark:text-stone-400 leading-relaxed text-base md:text-lg max-w-2xl">
                    {{ item.summary }}
                  </p>
                </div>
              </a>
            </article>
          </div>
        </section>
      </div>

      <!-- Footer -->
      <footer class="mt-32 pt-12 border-t border-stone-200 dark:border-stone-800 text-center text-sm text-stone-400 dark:text-stone-600 font-serif italic pb-12">
        <p>Generated by Potluck • {{ currentDate }}</p>
      </footer>
    </UContainer>
  </div>
</template>

<style scoped>
/* Scoped styles can remain empty if global styles cover everything */
</style>
