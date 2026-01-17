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

const heroStory = computed(() => todaysNews.value[0])
const otherStories = computed(() => todaysNews.value.slice(1))

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

      <main v-if="todaysNews.length > 0">
        <!-- Hero Section -->
        <article
          v-if="heroStory"
          class="mb-20 md:mb-24 group"
        >
          <a
            :href="heroStory.url"
            target="_blank"
            rel="noopener noreferrer"
            class="block"
          >
            <div class="flex items-center gap-3 text-sm font-medium uppercase tracking-wider text-stone-500 dark:text-stone-400 mb-4">
              <span class="text-orange-600 dark:text-orange-400 font-bold">Top Story</span>
              <span class="w-1 h-1 rounded-full bg-stone-300 dark:bg-stone-700" />
              <span>{{ formatTime(heroStory.publishedAt) }}</span>
              <span class="ml-auto bg-stone-100 dark:bg-stone-900 px-3 py-1 rounded-full text-stone-600 dark:text-stone-400 text-xs">{{ heroStory.tag }}</span>
            </div>

            <h2 class="text-5xl md:text-7xl font-serif font-bold text-stone-900 dark:text-stone-100 leading-[0.95] mb-6 group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
              {{ heroStory.title }}
            </h2>

            <p class="text-xl md:text-2xl text-stone-600 dark:text-stone-300 leading-relaxed font-serif max-w-3xl border-l-4 border-stone-200 dark:border-stone-800 pl-6">
              {{ heroStory.summary }}
            </p>

            <div class="mt-4 flex items-center gap-2 text-stone-400 text-sm font-medium pl-7 group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
              Read on {{ heroStory.source }} <UIcon
                name="i-lucide-arrow-right"
                class="w-4 h-4"
              />
            </div>
          </a>
        </article>

        <!-- Divider -->
        <div class="h-px bg-stone-200 dark:bg-stone-800 mb-16" />

        <!-- Remaining Stories List -->
        <div class="grid gap-12">
          <article
            v-for="(item, index) in otherStories"
            :key="index"
            class="group border-b border-stone-100 dark:border-stone-900/50 pb-12 last:border-0"
          >
            <a
              :href="item.url"
              target="_blank"
              rel="noopener noreferrer"
              class="block grid md:grid-cols-[1fr,3fr] gap-6 md:gap-12 items-baseline"
            >
              <div class="flex items-center gap-3 text-xs font-medium uppercase tracking-wider text-stone-400 dark:text-stone-500">
                <span class="text-orange-600 dark:text-orange-400">{{ formatTime(item.publishedAt) }}</span>
                <span class="text-stone-300 dark:text-stone-700">/</span>
                <span>{{ item.tag }}</span>
              </div>

              <div>
                <h3 class="text-2xl md:text-3xl font-serif font-bold text-stone-900 dark:text-stone-100 leading-tight mb-3 group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
                  {{ item.title }}
                </h3>
                <p class="text-stone-600 dark:text-stone-400 leading-relaxed text-base md:text-lg mb-2">
                  {{ item.summary }}
                </p>
                <div class="text-xs text-stone-400 font-medium flex items-center gap-1 group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">
                  {{ item.source }} <UIcon
                    name="i-lucide-arrow-up-right"
                    class="w-3 h-3"
                  />
                </div>
              </div>
            </a>
          </article>
        </div>

        <div class="mt-20 text-center">
          <NuxtLink
            to="/timeline"
            class="inline-flex items-center gap-2 text-lg font-serif italic text-stone-500 hover:text-orange-600 dark:text-stone-400 dark:hover:text-orange-400 transition-colors"
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
          color="orange"
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
