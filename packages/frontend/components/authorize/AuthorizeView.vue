<script setup lang="ts">

interface Props {
  username: string
  applicationName: string
  claims: string[]
}


withDefaults(defineProps<Props>(), {})


</script>


<template>
  <div class="flex flex-row w-full h-screen">
    <div class="w-60em max-w-80% h-75% m-a flex flex-col items-center b-1 b-gray-300 rd-8px">
      <h1 class="text-2xl font-bold font-100 flex flex-col justify-center items-center mt-1em mb-2em m-0.8em">Authorize {{ applicationName }} access to your:</h1>
      <div class="flex flex-col justify-start items-start w-80% gap-0.5em max-h-70%">
        <AuthorizeClaim v-for="claim in claims" :claim="claim" />
      </div>
    </div>
  </div>
</template>

