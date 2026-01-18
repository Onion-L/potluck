<script setup lang="ts">
import MarkdownIt from 'markdown-it'
import DOMPurify from 'isomorphic-dompurify'

const props = defineProps<{
  content: string
}>()

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true
})

// Configure DOMPurify hooks to sanitize dangerous protocols and add safe link attributes
DOMPurify.addHook('afterSanitizeAttributes', (node) => {
  if (node.tagName === 'A') {
    const href = node.getAttribute('href') || ''
    const trimmedHref = href.trim().toLowerCase()

    // Block dangerous protocols
    if (
      trimmedHref.startsWith('javascript:')
      || trimmedHref.startsWith('data:')
      || trimmedHref.startsWith('vbscript:')
    ) {
      node.setAttribute('href', '#')
    }

    // Add safe attributes for external links
    node.setAttribute('target', '_blank')
    node.setAttribute('rel', 'noopener noreferrer')
  }
})

const renderedHtml = computed(() => {
  const html = md.render(props.content)
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: ['p', 'strong', 'em', 'a', 'code', 'pre', 'blockquote', 'ul', 'ol', 'li', 'br', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'span'],
    ALLOWED_ATTR: ['href', 'target', 'rel'],
    ALLOW_DATA_ATTR: false
  })
})
</script>

<template>
  <div
    class="prose prose-stone dark:prose-invert prose-p:my-0 prose-a:text-stone-900 dark:prose-a:text-stone-100 prose-strong:text-stone-900 dark:prose-strong:text-stone-100 prose-headings:text-stone-900 dark:prose-headings:text-stone-100 prose-list-item:text-stone-600 dark:prose-list-item:text-stone-400 prose-code:text-stone-600 dark:prose-code:text-stone-400"
    v-html="renderedHtml"
  />
</template>
