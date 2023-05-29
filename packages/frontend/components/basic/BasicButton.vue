<script setup lang="ts">
interface Props {
  text: string
}

const props = defineProps<Props>()
</script>

<template>
  <button class="px-5 py-5 bg-primary rounded-md">
    <p class="text-black font-semibold text-lg">
      {{ text }}
    </p>
  </button>
</template>
