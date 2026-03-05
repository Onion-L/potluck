<script setup lang="ts">
interface Props {
  title: string
  url: string
  variant?: 'default' | 'ghost' | 'subtle'
  size?: 'sm' | 'md'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  size: 'md'
})

const toast = useToast()
const isCopied = ref(false)

const handleClick = async (event: MouseEvent) => {
  // 阻止事件冒泡，防止触发父级链接跳转
  event.stopPropagation()
  event.preventDefault()

  const prompt = `Summarize the main content of this article: ${props.title}\n\nArticle link: ${props.url}`

  // 复制到剪贴板
  try {
    await navigator.clipboard.writeText(prompt)
    isCopied.value = true
    toast.add({
      title: 'Copied to clipboard',
      description: 'Opening z.ai...',
      icon: 'i-lucide-check',
      color: 'success'
    })
    setTimeout(() => {
      isCopied.value = false
    }, 2000)
  } catch {
    toast.add({
      title: 'Copy failed',
      description: 'Opening z.ai, please paste manually',
      icon: 'i-lucide-alert-circle',
      color: 'warning'
    })
  }

  // Open z.ai
  window.open('https://z.ai', '_blank', 'noopener,noreferrer')
}

const variantClasses = {
  default: 'bg-stone-900 dark:bg-stone-100 text-stone-50 dark:text-stone-900 hover:bg-stone-700 dark:hover:bg-stone-300',
  ghost: 'text-stone-500 dark:text-stone-400 hover:text-stone-900 dark:hover:text-stone-100 hover:bg-stone-100 dark:hover:bg-stone-900',
  subtle: 'bg-stone-100 dark:bg-stone-900 text-stone-700 dark:text-stone-300 hover:bg-stone-200 dark:hover:bg-stone-800'
}

const sizeClasses = {
  sm: 'px-2.5 py-1.5 text-xs gap-1',
  md: 'px-3 py-2 text-sm gap-1.5'
}

const iconSizes = {
  sm: 'w-3.5 h-3.5',
  md: 'w-4 h-4'
}
</script>

<template>
  <button
    class="inline-flex items-center font-medium rounded-lg transition-all duration-200 cursor-pointer hover:scale-105 active:scale-95"
    :class="[variantClasses[variant], sizeClasses[size]]"
    @click="handleClick"
  >
    <UIcon
      :name="isCopied ? 'i-lucide-check' : 'i-lucide-sparkles'"
      :class="iconSizes[size]"
    />
    <span>{{ isCopied ? 'Copied' : 'AI Summary' }}</span>
  </button>
</template>
