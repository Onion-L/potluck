<script setup lang="ts">
import newsData from '~/assets/data/latest.json'

interface _NewsItem {
  title: string
  url: string
  summary: string
  tag: string
  source: string
  publishedAt: string
}

const news = ref(newsData)

// Get today's date in YYYY-MM-DD
// Note: In a real app we'd want to be careful about server/client timezone mismatch
// For this mock data context, we'll align with the mock data's "today" which is 2026-01-17
const TODAY_DATE = '2026-01-17'

const todaysNews = computed(() => {
  if (!news.value) return []

  return news.value
    .filter(item => item.publishedAt.startsWith(TODAY_DATE))
    .sort((a, b) => new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime())
})

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
  <div class="min-h-screen bg-stone-50 dark:bg-stone-950 font-sans selection:bg-stone-200 dark:selection:bg-stone-800 transition-colors duration-300 bg-noise">
    <UContainer class="py-12 md:py-20 relative z-10 max-w-4xl">
      <!-- Header -->
      <AppHeader />

      <main v-if="todaysNews.length > 0">
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
                <span class="font-sans text-xs font-bold text-stone-400 uppercase shrink-0 tracking-tighter">
                  {{ formatTime(item.publishedAt) }}
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
              class="block grid md:grid-cols-[1fr,3fr] gap-6 md:gap-12 items-baseline p-6 -m-6 rounded-2xl transition-all duration-300 group-hover:translate-x-1"
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
                <p class="text-stone-600 dark:text-stone-400 leading-relaxed text-base md:text-lg mb-2">
                  {{ item.summary }}
                </p>
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

      <div
        v-else
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

      <!-- Footer -->
      <footer class="mt-32 pt-12 border-t border-stone-200 dark:border-stone-800 text-center text-sm text-stone-400 dark:text-stone-600 font-serif italic pb-12">
        <p>Generated by Potluck • {{ currentDate }}</p>
      </footer>
    </UContainer>
  </div>
</template>
