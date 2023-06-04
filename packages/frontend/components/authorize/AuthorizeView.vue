<script setup lang="ts">
import BasicButton from '../basic/BasicButton.vue';


interface Props {
  username: string
  applicationName: string
  claims: string[]
}


withDefaults(defineProps<Props>(), {})


</script>


<template>
  <div class="flex flex-row w-full min-h-screen h-fit">
    <div class="w-50em sm:max-w-80% h-fit m-a flex flex-col items-center b-1 b-dark-300 rd-8px p-3 w-90% sm:p-10" >
      <h1 class="text-2xl font-bold font-100 flex flex-col justify-center items-center mb-2em ">Authorize {{
        applicationName }} access to your:</h1>
      <div class="flex flex-col justify-start items-start w-100% gap-0.5em max-h-70%">
        <AuthorizeClaim v-for="claim in claims" :claim="claim" />
      </div>
      <div class="flex flex-row flex-wrap gap-8 w-full justify-center m-t-12">
        <div class="flex sm:w-50 w-100%">
          <BasicButton text="Deny" variant="outline" />
        </div>
        <div class="flex sm:w-50 w-100%">
          <BasicButton text="Authorize" />
        </div>
      </div>
    </div>
  </div>
</template>

