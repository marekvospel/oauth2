<script setup lang="ts">
interface Props {
  type?: 'submit' | 'button'
  variant?: 'fill' | 'outline'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  variant: 'fill'
})
</script>

<template>
  <button :type="type" class="px-4 py-4 font-semibold text-lg rounded-md w-full"
  :class="{
    'variant-fill': variant === 'fill',
    'variant-outline': variant === 'outline',
  }">
    <slot />
  </button>
</template>

<style scoped>
.variant-fill {
  @apply bg-primary text-black;
}

.variant-outline {
  @apply border-solid border-2 border-primary text-primary;
}
</style>