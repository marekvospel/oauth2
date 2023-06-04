<script setup lang="ts">
interface Props {
  text: string
  type?: 'submit' | 'button'
  variant?: 'fill' | 'outline'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  variant: 'fill'
})

const mainStyle = computed(() => {
  if (props.variant === 'fill') {
    return 'bg-primary text-black '
  } else {
    return 'border-solid border-2 border-primary text-primary'
  }
})

</script>

<template>
  <button :type="type" :class="mainStyle" class="px-3 py-4 font-semibold variant text-lg rounded-md w-100%">
    {{ text }}
  </button>
</template>
