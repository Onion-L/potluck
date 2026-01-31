<script setup lang="ts">
import { ref, onMounted } from 'vue'

const colorMode = useColorMode()
const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set() {
    colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
  }
})

// Animation state
const isLoaded = ref(false)
onMounted(() => {
  setTimeout(() => {
    isLoaded.value = true
  }, 100)
})

const features = [
  {
    title: 'AI-Powered Curation',
    description: 'Advanced filtering rejects noise — minor updates, marketing content, beginner tutorials. Only high-value technical insights make the cut.',
    icon: 'i-lucide-brain'
  },
  {
    title: 'Intelligent Summaries',
    description: 'Each article is distilled into concise, actionable summaries in Chinese. Understand the impact without reading the full piece.',
    icon: 'i-lucide-file-text'
  },
  {
    title: 'Multi-Source Aggregation',
    description: 'Curated from top tech RSS feeds worldwide. One place for all the news that matters to developers and tech enthusiasts.',
    icon: 'i-lucide-rss'
  }
]

const workflow = [
  { step: '01', title: 'Fetch', description: 'RSS feeds from trusted tech sources' },
  { step: '02', title: 'Filter', description: 'AI gatekeeper evaluates importance' },
  { step: '03', title: 'Summarize', description: 'Generate concise Chinese summaries' },
  { step: '04', title: 'Deliver', description: 'Clean, readable news digest' }
]
</script>

<template>
  <div class="min-h-screen bg-stone-50 dark:bg-stone-950 font-sans selection:bg-stone-200 dark:selection:bg-stone-800 transition-colors duration-300 bg-noise">
    <!-- Header - Consistent with main site -->
    <header class="fixed top-0 left-0 right-0 z-40 bg-stone-50/80 dark:bg-stone-950/80 backdrop-blur-sm border-b border-stone-200 dark:border-stone-800">
      <UContainer class="max-w-4xl py-3 sm:py-4 px-4 sm:px-6 flex items-center justify-between">
        <NuxtLink
          to="/"
          class="font-serif font-bold text-xl sm:text-2xl text-stone-900 dark:text-stone-100 hover:text-stone-600 dark:hover:text-stone-300 transition-colors min-h-[44px] flex items-center"
        >
          POTLUCK
        </NuxtLink>
        <div class="flex items-center gap-2 sm:gap-4">
          <NuxtLink
            to="/today"
            class="text-sm font-medium text-stone-500 hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-100 transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center"
          >
            Today
          </NuxtLink>
          <NuxtLink
            to="/timeline"
            class="text-sm font-medium text-stone-500 hover:text-stone-900 dark:text-stone-400 dark:hover:text-stone-100 transition-colors min-h-[44px] min-w-[44px] flex items-center justify-center"
          >
            Timeline
          </NuxtLink>
          <div class="h-4 w-px bg-stone-300 dark:bg-stone-700 hidden sm:block" />
          <ClientOnly>
            <UButton
              :icon="isDark ? 'i-heroicons-moon-20-solid' : 'i-heroicons-sun-20-solid'"
              color="neutral"
              variant="ghost"
              size="sm"
              aria-label="Toggle theme"
              class="min-h-[44px] min-w-[44px]"
              @click="isDark = !isDark"
            />
          </ClientOnly>
        </div>
      </UContainer>
    </header>

    <!-- Hero Section -->
    <section class="min-h-screen flex items-center justify-center relative overflow-hidden">
      <!-- Animated Background Lines -->
      <div class="absolute inset-0 pointer-events-none overflow-hidden">
        <!-- Horizontal lines -->
        <div
          v-for="i in 5"
          :key="`h-${i}`"
          class="absolute left-0 right-0 h-px bg-stone-200 dark:bg-stone-800 origin-left transition-transform duration-1000 ease-out"
          :class="isLoaded ? 'scale-x-100' : 'scale-x-0'"
          :style="{
            top: `${15 + i * 15}%`,
            transitionDelay: `${i * 100}ms`
          }"
        />
        <!-- Vertical accent line -->
        <div
          class="absolute left-1/2 top-0 bottom-0 w-px bg-linear-to-b from-transparent via-stone-300 dark:via-stone-700 to-transparent origin-top transition-transform duration-1000 delay-300"
          :class="isLoaded ? 'scale-y-100' : 'scale-y-0'"
        />
      </div>

      <UContainer class="max-w-4xl relative z-10 py-24 sm:py-32 px-4 sm:px-6">
        <div class="text-center">
          <!-- Masthead style -->
          <div
            class="mb-6 sm:mb-8 opacity-0 transition-all duration-700 ease-out"
            :class="{ 'opacity-100': isLoaded }"
          >
            <div class="inline-flex items-center gap-3 sm:gap-4">
              <div class="w-8 sm:w-12 h-px bg-stone-400 dark:bg-stone-600" />
              <span class="text-xs uppercase tracking-[0.2em] sm:tracking-[0.3em] text-stone-500 dark:text-stone-400 font-medium">
                Est. 2024
              </span>
              <div class="w-8 sm:w-12 h-px bg-stone-400 dark:bg-stone-600" />
            </div>
          </div>

          <!-- Main Title - Newspaper masthead style -->
          <h1
            class="font-serif font-bold text-stone-900 dark:text-stone-100 mb-4 sm:mb-6 opacity-0 translate-y-8 transition-all duration-700 delay-200"
            :class="{ 'opacity-100 translate-y-0': isLoaded }"
          >
            <span class="block text-[clamp(3rem,12vw,10rem)] sm:text-[clamp(4rem,15vw,10rem)] leading-[0.85] tracking-tight">
              POTLUCK
            </span>
          </h1>

          <!-- Decorative divider -->
          <div
            class="flex items-center justify-center gap-2 sm:gap-3 mb-6 sm:mb-8 opacity-0 transition-all duration-500 delay-400"
            :class="{ 'opacity-100': isLoaded }"
          >
            <div class="w-12 sm:w-16 md:w-24 h-px bg-stone-300 dark:bg-stone-700" />
            <div class="w-1.5 h-1.5 rounded-full bg-stone-400 dark:bg-stone-600" />
            <div class="w-12 sm:w-16 md:w-24 h-px bg-stone-300 dark:bg-stone-700" />
          </div>

          <!-- Tagline -->
          <p
            class="text-lg sm:text-xl md:text-2xl lg:text-3xl font-serif italic text-stone-600 dark:text-stone-400 mb-3 sm:mb-4 opacity-0 translate-y-4 transition-all duration-700 delay-500"
            :class="{ 'opacity-100 translate-y-0': isLoaded }"
          >
            Your Daily Tech Intelligence
          </p>

          <!-- Subtitle -->
          <p
            class="text-sm sm:text-base md:text-lg text-stone-500 dark:text-stone-500 max-w-lg mx-auto mb-8 sm:mb-12 leading-relaxed px-4 sm:px-0 opacity-0 translate-y-4 transition-all duration-700 delay-600"
            :class="{ 'opacity-100 translate-y-0': isLoaded }"
          >
            AI-curated news digest. Filtered for signal. Summarized in Chinese.
          </p>

          <!-- Feature Pills -->
          <div
            class="flex flex-wrap items-center justify-center gap-2 sm:gap-3 mb-8 sm:mb-12 opacity-0 translate-y-4 transition-all duration-700 delay-700"
            :class="{ 'opacity-100 translate-y-0': isLoaded }"
          >
            <span class="px-3 sm:px-4 py-2 text-xs uppercase tracking-wider font-medium text-stone-600 dark:text-stone-400 border border-dashed border-stone-300 dark:border-stone-700 rounded-full">
              50+ Sources
            </span>
            <span class="px-3 sm:px-4 py-2 text-xs uppercase tracking-wider font-medium text-stone-600 dark:text-stone-400 border border-dashed border-stone-300 dark:border-stone-700 rounded-full">
              AI Filtered
            </span>
            <span class="px-3 sm:px-4 py-2 text-xs uppercase tracking-wider font-medium text-stone-600 dark:text-stone-400 border border-dashed border-stone-300 dark:border-stone-700 rounded-full">
              中文摘要
            </span>
          </div>

          <!-- CTA Buttons -->
          <div
            class="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 opacity-0 translate-y-4 transition-all duration-700 delay-800"
            :class="{ 'opacity-100 translate-y-0': isLoaded }"
          >
            <NuxtLink
              to="/today"
              class="group inline-flex items-center gap-2 px-6 sm:px-8 py-3.5 sm:py-4 bg-stone-900 dark:bg-stone-100 text-stone-50 dark:text-stone-900 font-medium rounded-full hover:bg-stone-700 dark:hover:bg-stone-300 transition-all duration-300 cursor-pointer hover:scale-105 min-h-[48px] sm:min-h-[56px]"
            >
              Read Today's Edition
              <UIcon
                name="i-lucide-arrow-right"
                class="w-5 h-5 transition-transform duration-300 group-hover:translate-x-1"
              />
            </NuxtLink>
            <NuxtLink
              to="/timeline"
              class="group inline-flex items-center gap-2 px-6 sm:px-8 py-3.5 sm:py-4 border border-stone-300 dark:border-stone-700 text-stone-700 dark:text-stone-300 font-medium rounded-full hover:border-stone-500 dark:hover:border-stone-500 hover:text-stone-900 dark:hover:text-stone-100 transition-all duration-300 cursor-pointer min-h-[48px] sm:min-h-[56px]"
            >
              Browse Archives
            </NuxtLink>
          </div>
        </div>

        <!-- Scroll hint -->
        <div
          class="absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 opacity-0 transition-all duration-700 delay-1000"
          :class="{ 'opacity-100': isLoaded }"
        >
          <span class="text-xs uppercase tracking-widest text-stone-400 dark:text-stone-600 font-medium">Scroll</span>
          <div class="w-px h-8 bg-linear-to-b from-stone-400 dark:from-stone-600 to-transparent animate-bounce-subtle" />
        </div>
      </UContainer>
    </section>

    <!-- Section Divider -->
    <div class="max-w-4xl mx-auto px-4 sm:px-6">
      <div class="flex items-center gap-3 sm:gap-4 py-4">
        <div class="flex-1 border-t border-dashed border-stone-300 dark:border-stone-700" />
        <span class="text-xs uppercase tracking-widest text-stone-400 dark:text-stone-600 font-medium">About</span>
        <div class="flex-1 border-t border-dashed border-stone-300 dark:border-stone-700" />
      </div>
    </div>

    <!-- Features Section -->
    <section class="py-16 sm:py-20 md:py-28">
      <UContainer class="max-w-4xl px-4 sm:px-6">
        <div class="text-center mb-12 sm:mb-16">
          <h2 class="text-2xl sm:text-3xl md:text-4xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-3 sm:mb-4">
            Why Potluck?
          </h2>
          <p class="text-stone-500 dark:text-stone-400 max-w-lg mx-auto font-serif italic px-4 sm:px-0">
            Not another feed reader. A smarter way to stay informed.
          </p>
        </div>

        <div class="grid sm:grid-cols-2 md:grid-cols-3 gap-4 sm:gap-6 md:gap-8">
          <div
            v-for="feature in features"
            :key="feature.title"
            class="group p-4 sm:p-6 border border-dashed border-stone-200 dark:border-stone-800 rounded-2xl hover:border-stone-400 dark:hover:border-stone-600 transition-all duration-300 cursor-pointer hover:-translate-y-1"
          >
            <div class="w-10 h-10 sm:w-12 sm:h-12 rounded-full bg-stone-100 dark:bg-stone-900 flex items-center justify-center mb-4 sm:mb-6 group-hover:bg-stone-200 dark:group-hover:bg-stone-800 transition-colors duration-300">
              <UIcon
                :name="feature.icon"
                class="w-5 h-5 sm:w-6 sm:h-6 text-stone-600 dark:text-stone-400"
              />
            </div>
            <h3 class="text-lg sm:text-xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-2 sm:mb-3">
              {{ feature.title }}
            </h3>
            <p class="text-stone-600 dark:text-stone-400 leading-relaxed text-sm">
              {{ feature.description }}
            </p>
          </div>
        </div>
      </UContainer>
    </section>

    <!-- How It Works -->
    <section class="py-16 sm:py-20 md:py-28 border-t border-dashed border-stone-200 dark:border-stone-800">
      <UContainer class="max-w-4xl px-4 sm:px-6">
        <div class="text-center mb-12 sm:mb-16">
          <h2 class="text-2xl sm:text-3xl md:text-4xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-3 sm:mb-4">
            How It Works
          </h2>
          <p class="text-stone-500 dark:text-stone-400 max-w-lg mx-auto font-serif italic px-4 sm:px-0">
            From raw feeds to polished digest in four steps.
          </p>
        </div>

        <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
          <div
            v-for="(item, index) in workflow"
            :key="item.step"
            class="group relative cursor-pointer"
          >
            <!-- Connector line -->
            <div
              v-if="index < workflow.length - 1"
              class="hidden lg:block absolute top-6 left-[calc(50%+2rem)] w-[calc(100%-4rem)] border-t border-dashed border-stone-300 dark:border-stone-700"
            />
            <div class="text-center p-3 sm:p-6 transition-transform duration-300 group-hover:-translate-y-1">
              <div class="text-3xl sm:text-4xl font-serif font-bold text-stone-200 dark:text-stone-800 mb-2 sm:mb-3 transition-colors duration-300 group-hover:text-stone-300 dark:group-hover:text-stone-700">
                {{ item.step }}
              </div>
              <h3 class="text-base sm:text-lg font-serif font-bold text-stone-900 dark:text-stone-100 mb-1 sm:mb-2">
                {{ item.title }}
              </h3>
              <p class="text-xs sm:text-sm text-stone-500 dark:text-stone-400">
                {{ item.description }}
              </p>
            </div>
          </div>
        </div>
      </UContainer>
    </section>

    <!-- Tech Stack -->
    <section class="py-16 sm:py-20 md:py-28 border-t border-dashed border-stone-200 dark:border-stone-800">
      <UContainer class="max-w-4xl px-4 sm:px-6">
        <div class="text-center mb-12 sm:mb-16">
          <h2 class="text-2xl sm:text-3xl md:text-4xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-3 sm:mb-4">
            Built With
          </h2>
          <p class="text-stone-500 dark:text-stone-400 max-w-lg mx-auto font-serif italic px-4 sm:px-0">
            Modern stack for reliability and performance.
          </p>
        </div>

        <div class="flex flex-wrap items-center justify-center gap-4 sm:gap-6 md:gap-10">
          <!-- Nuxt -->
          <div class="group flex items-center gap-2 cursor-pointer transition-all duration-300 min-h-[44px]">
            <UIcon
              name="i-simple-icons-nuxtdotjs"
              class="w-5 h-5 sm:w-6 sm:h-6 text-stone-400 dark:text-stone-500 transition-all duration-300 group-hover:text-[#00DC82] group-hover:drop-shadow-[0_0_8px_#00DC82]"
            />
            <span class="text-xs sm:text-sm font-medium text-stone-400 dark:text-stone-500 transition-colors duration-300 group-hover:text-stone-600 dark:group-hover:text-stone-300">Nuxt</span>
          </div>
          <!-- Supabase -->
          <div class="group flex items-center gap-2 cursor-pointer transition-all duration-300 min-h-[44px]">
            <UIcon
              name="i-simple-icons-supabase"
              class="w-5 h-5 sm:w-6 sm:h-6 text-stone-400 dark:text-stone-500 transition-all duration-300 group-hover:text-[#3ECF8E] group-hover:drop-shadow-[0_0_8px_#3ECF8E]"
            />
            <span class="text-xs sm:text-sm font-medium text-stone-400 dark:text-stone-500 transition-colors duration-300 group-hover:text-stone-600 dark:group-hover:text-stone-300">Supabase</span>
          </div>
          <!-- Tailwind -->
          <div class="group flex items-center gap-2 cursor-pointer transition-all duration-300 min-h-[44px]">
            <UIcon
              name="i-simple-icons-tailwindcss"
              class="w-5 h-5 sm:w-6 sm:h-6 text-stone-400 dark:text-stone-500 transition-all duration-300 group-hover:text-[#06B6D4] group-hover:drop-shadow-[0_0_8px_#06B6D4]"
            />
            <span class="text-xs sm:text-sm font-medium text-stone-400 dark:text-stone-500 transition-colors duration-300 group-hover:text-stone-600 dark:group-hover:text-stone-300">Tailwind</span>
          </div>
          <!-- DeepSeek -->
          <div class="group flex items-center gap-2 cursor-pointer transition-all duration-300 min-h-[44px]">
            <svg
              class="w-5 h-5 sm:w-6 sm:h-6 text-stone-400 dark:text-stone-500 transition-all duration-300 group-hover:text-[#4D6BFE] group-hover:drop-shadow-[0_0_8px_#4D6BFE]"
              viewBox="0 0 377.1 277.86"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M373.15,23.32c-4-1.95-5.72,1.77-8.06,3.66-.79.62-1.47,1.43-2.14,2.14-5.85,6.26-12.67,10.36-21.57,9.86-13.04-.71-24.16,3.38-33.99,13.37-2.09-12.31-9.04-19.66-19.6-24.38-5.54-2.45-11.13-4.9-14.99-10.23-2.71-3.78-3.44-8-4.81-12.16-.85-2.51-1.72-5.09-4.6-5.52-3.13-.5-4.36,2.14-5.58,4.34-4.93,8.99-6.82,18.92-6.65,28.97.43,22.58,9.97,40.56,28.89,53.37,2.16,1.46,2.71,2.95,2.03,5.09-1.29,4.4-2.82,8.68-4.19,13.09-.85,2.82-2.14,3.44-5.15,2.2-10.39-4.34-19.37-10.76-27.29-18.55-13.46-13.02-25.63-27.41-40.81-38.67-3.57-2.64-7.12-5.09-10.81-7.41-15.49-15.07,2.03-27.45,6.08-28.9,4.25-1.52,1.47-6.79-12.23-6.73-13.69.06-26.24,4.65-42.21,10.76-2.34.93-4.79,1.61-7.32,2.14-14.5-2.73-29.55-3.35-45.29-1.58-29.62,3.32-53.28,17.34-70.68,41.28C1.29,88.2-3.63,120.88,2.39,155c6.33,35.91,24.64,65.68,52.8,88.94,29.18,24.1,62.8,35.91,101.15,33.65,23.29-1.33,49.23-4.46,78.48-29.24,7.38,3.66,15.12,5.12,27.97,6.23,9.89.93,19.41-.5,26.79-2.02,11.55-2.45,10.75-13.15,6.58-15.13-33.87-15.78-26.44-9.36-33.2-14.54,17.21-20.41,43.15-41.59,53.3-110.19.79-5.46.11-8.87,0-13.3-.06-2.67.54-3.72,3.61-4.03,8.48-.96,16.72-3.29,24.28-7.47,21.94-12,30.78-31.69,32.87-55.33.31-3.6-.06-7.35-3.86-9.24ZM181.96,235.97c-32.83-25.83-48.74-34.33-55.31-33.96-6.14.34-5.04,7.38-3.69,11.97,1.41,4.53,3.26,7.66,5.85,11.63,1.78,2.64,3.01,6.57-1.78,9.49-10.57,6.58-28.95-2.2-29.82-2.64-21.38-12.59-39.26-29.24-51.87-52.01-12.16-21.92-19.23-45.43-20.39-70.52-.31-6.08,1.47-8.22,7.49-9.3,7.92-1.46,16.11-1.77,24.03-.62,33.49,4.9,62.01,19.91,85.9,43.63,13.65,13.55,23.97,29.71,34.61,45.49,11.3,16.78,23.48,32.75,38.97,45.84,5.46,4.59,9.83,8.09,14,10.67-12.59,1.4-33.62,1.71-47.99-9.68ZM197.69,134.65c0-2.7,2.15-4.84,4.87-4.84.6,0,1.16.12,1.66.31.67.25,1.29.62,1.77,1.18.87.84,1.36,2.08,1.36,3.35,0,2.7-2.15,4.84-4.85,4.84s-4.81-2.14-4.81-4.84ZM246.55,159.77c-3.13,1.27-6.26,2.39-9.27,2.51-4.67.22-9.77-1.68-12.55-4-4.3-3.6-7.36-5.61-8.67-11.94-.54-2.7-.23-6.85.25-9.24,1.12-5.15-.12-8.44-3.74-11.44-2.96-2.45-6.7-3.1-10.82-3.1-1.54,0-2.95-.68-4-1.24-1.72-.87-3.13-3.01-1.78-5.64.43-.84,2.53-2.92,3.02-3.29,5.58-3.19,12.03-2.14,18,.25,5.54,2.26,9.71,6.42,15.72,12.28,6.16,7.1,7.26,9.09,10.76,14.39,2.76,4.19,5.29,8.47,7.01,13.37,1.04,3.04-.31,5.55-3.94,7.1Z" />
            </svg>
            <span class="text-xs sm:text-sm font-medium text-stone-400 dark:text-stone-500 transition-colors duration-300 group-hover:text-stone-600 dark:group-hover:text-stone-300">DeepSeek</span>
          </div>
        </div>
      </UContainer>
    </section>

    <!-- CTA Section -->
    <section class="py-16 sm:py-20 md:py-28 text-center border-t border-dashed border-stone-200 dark:border-stone-800">
      <UContainer class="max-w-4xl px-4 sm:px-6">
        <h2 class="text-2xl sm:text-3xl md:text-5xl font-serif font-bold text-stone-900 dark:text-stone-100 mb-4 sm:mb-6">
          Ready to read smarter?
        </h2>
        <p class="text-base sm:text-lg text-stone-600 dark:text-stone-400 max-w-xl mx-auto mb-8 sm:mb-10 font-serif italic px-4 sm:px-0">
          No signup required. Just pure, curated tech news.
        </p>
        <NuxtLink
          to="/today"
          class="group inline-flex items-center gap-2 px-8 sm:px-10 py-4 sm:py-5 bg-stone-900 dark:bg-stone-100 text-stone-50 dark:text-stone-900 text-base sm:text-lg font-medium rounded-full hover:bg-stone-700 dark:hover:bg-stone-300 transition-all duration-300 cursor-pointer hover:scale-105 min-h-[52px] sm:min-h-[60px]"
        >
          Start Reading
          <UIcon
            name="i-lucide-arrow-right"
            class="w-5 h-5 transition-transform duration-300 group-hover:translate-x-1"
          />
        </NuxtLink>
      </UContainer>
    </section>

    <!-- Footer -->
    <footer class="py-8 sm:py-12 border-t border-stone-200 dark:border-stone-800">
      <UContainer class="max-w-4xl px-4 sm:px-6 text-center">
        <p class="text-sm text-stone-400 dark:text-stone-600 font-serif italic">
          Open source project by
          <a
            href="https://github.com/Onion-L"
            target="_blank"
            rel="noopener noreferrer"
            class="hover:text-stone-600 dark:hover:text-stone-400 underline underline-offset-2 transition-colors"
          >
            Onion-L
          </a>
        </p>
      </UContainer>
    </footer>
  </div>
</template>

<style scoped>
@keyframes bounce-subtle {
  0%, 100% {
    transform: translateY(0);
    opacity: 1;
  }
  50% {
    transform: translateY(4px);
    opacity: 0.5;
  }
}

.animate-bounce-subtle {
  animation: bounce-subtle 2s ease-in-out infinite;
}

/* Respect reduced motion preference */
@media (prefers-reduced-motion: reduce) {
  .animate-bounce-subtle {
    animation: none;
  }

  * {
    transition-duration: 0.01ms !important;
    animation-duration: 0.01ms !important;
  }
}
</style>
