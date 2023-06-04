<script setup lang="ts">
interface Props {
  modelValue?: string
  placeholder: string
  label?: string
  type?: 'password' | 'text' | 'textarea' | 'email' | 'tel' | 'number'
}

const props = defineProps<Props>()
const emit = defineEmits(['update:modelValue'])

const value = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})
</script>

<template>
  <div class="flex flex-col gap-1 w-full items-start">
    <p class="text-xl font-normal">
      {{ label }}
    </p>
    <input v-model="value" :placeholder="placeholder" :type="type ?? 'text'" class="px-3 py-2 text-lg text-black rounded-md w-full">
  </div>
</template>
