<script setup lang="ts">
const currentDate = new Date().toLocaleDateString('en-US', {
  weekday: 'long',
  year: 'numeric',
  month: 'long',
  day: 'numeric'
})

const colorMode = useColorMode()
const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set() {
    colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
  }
})

const route = useRoute()
const isToday = computed(() => route.path === '/today')
const isTimeline = computed(() => route.path === '/timeline')
</script>

<template>
  <header class="mb-6 md:mb-8 flex flex-col md:flex-row md:items-end justify-between gap-8 border-b border-stone-200 dark:border-stone-800 pb-8">
    <div class="text-center md:text-left">
      <h1 class="text-6xl md:text-8xl font-serif font-bold tracking-tight text-stone-900 dark:text-stone-100 mb-4">
        <NuxtLink to="/">POTLUCK</NuxtLink>
      </h1>
      <div class="flex flex-col md:flex-row md:items-center gap-4 text-stone-500 dark:text-stone-400 pt-2">
        <span class="uppercase tracking-widest text-sm font-medium">{{ currentDate }}</span>
        <span class="hidden md:inline text-stone-300 dark:text-stone-700">•</span>
        <span class="text-sm italic font-serif">Daily News Aggregator</span>
      </div>
    </div>

    <div class="flex items-center gap-6 self-center md:self-end md:mb-4">
      <nav class="flex items-center gap-6">
        <NuxtLink
          to="/today"
          class="text-sm tracking-wide transition-all duration-300 decoration-2 underline-offset-4"
          :class="isToday ? 'font-bold text-stone-900 underline dark:text-stone-100' : 'font-medium text-stone-500 hover:text-stone-900 hover:underline dark:text-stone-400 dark:hover:text-stone-50'"
        >
          Today
        </NuxtLink>
        <NuxtLink
          to="/timeline"
          class="text-sm tracking-wide transition-all duration-300 decoration-2 underline-offset-4"
          :class="isTimeline ? 'font-bold text-stone-900 underline dark:text-stone-100' : 'font-medium text-stone-500 hover:text-stone-900 hover:underline dark:text-stone-400 dark:hover:text-stone-50'"
        >
          Timeline
        </NuxtLink>
      </nav>

      <div class="h-4 w-px bg-stone-300 dark:bg-stone-700" />

      <ClientOnly>
        <UButton
          :icon="isDark ? 'i-heroicons-moon-20-solid' : 'i-heroicons-sun-20-solid'"
          color="neutral"
          variant="ghost"
          aria-label="Theme"
          @click="isDark = !isDark"
        />
      </ClientOnly>
    </div>
  </header>
</template>
