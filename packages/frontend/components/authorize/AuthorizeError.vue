<script setup lang="ts">
interface Props {
  error?: string
}

withDefaults(defineProps<Props>(), {
  error: 'Unknown application',
})
</script>

<template>
  <div>
    <p>{{ error }}</p>
  </div>
</template>
