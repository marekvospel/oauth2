<script setup lang="ts">
interface Props {
  modelValue?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits(['update:modelValue'])

const value = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
})
</script>

<template>
  <div class="flex flex-row gap-4 items-center">
    <input v-model="value" type="checkbox" class="px-3 py-2 text-lg text-black rounded-md">
    <p class="text-lg font-normal">
      I agree with your
      <a href="/privacy-policy" class="text-primary">
        policy
      </a>
      bullshit
    </p>
  </div>
</template>
