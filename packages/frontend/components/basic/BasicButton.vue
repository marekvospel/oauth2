<script setup lang="ts">
interface Props {
  type?: 'submit' | 'button'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
})
</script>

<template>
  <button :type="type" class="px-5 py-5 bg-primary rounded-md text-black font-semibold text-lg">
    <slot />
  </button>
</template>
