<script setup lang="ts">
interface Props {
  modelValue?: string
  placeholder: string
  label: string
}

const props = defineProps<Props>()
const emit = defineEmits(['update:modelValue'])

const value = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})
</script>

<template>
  <div>
    <p class="text-xl font-normal">
      {{ label }}
    </p>

    <input v-model="value" :placeholder="placeholder" class="px-3 py-2 text-lg text-black rounded-md w-full">
  </div>
</template>
