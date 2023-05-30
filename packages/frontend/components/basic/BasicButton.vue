<script setup lang="ts">
interface Props {
  text: string
  type?: 'submit' | 'button'
}

const props = defineProps<Props>()
</script>

<template>
  <button :type="type ?? 'button'" class="px-5 py-5 bg-primary rounded-md text-black font-semibold text-lg">
    {{ text }}
  </button>
</template>
