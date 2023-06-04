<script setup lang="ts">
interface Props {
  text: string
  type?: 'submit' | 'button'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
})
</script>

<template>
  <button :type="type" class="px-5 py-5 bg-primary rounded-md text-black font-semibold text-lg">
    {{ text }}
  </button>
</template>
