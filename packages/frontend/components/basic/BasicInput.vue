<script setup lang="ts">
interface Props {
  modelValue?: string
  placeholder: string
  label?: string
  type?: string
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
    <p v-if="type !== 'agree'" class="text-xl font-normal">
      {{ label }}
    </p>
    <input v-if="type !== 'agree'" v-model="value" :placeholder="placeholder" :type="type ?? 'text'" class="px-3 py-2 text-lg text-black rounded-md w-full">
    <div v-else class="flex flex-row gap-4 items-center">
      <input v-model="value" :placeholder="placeholder" type="checkbox" class="px-3 py-2 text-lg text-black rounded-md">
      <p class="text-lg font-normal">
        I agree with your
        <a href="/privacy-policy" class="text-primary">
          policy
        </a>
        bullshit
      </p>
    </div>
  </div>
</template>
