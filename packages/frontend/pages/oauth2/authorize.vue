<script setup lang="ts">
const route = useRoute()

const scopes = computed(() => (route.query.scope?.toString() ?? '').split('\s+').filter(Boolean))
</script>

<template>
  <form method="post">
    <h1>Authorize {app name}</h1>
    <ul v-if="scopes.length >= 1">
      <li v-for="scope of scopes" :key="scope">
        {{ scope }}
      </li>
    </ul>
    <p v-else>No scopes selected</p>
    <div>
      <button type="cancel">Cancel</button>
      <button type="submit">Authorize</button>
    </div>
  </form>
</template>