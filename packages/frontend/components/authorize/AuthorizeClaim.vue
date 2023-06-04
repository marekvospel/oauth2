<script setup lang="ts">



interface Props {
  claim: keyof typeof claims,
}

const props = withDefaults(defineProps<Props>(), {})

const claims = {
    "identity": {
        "header": "Identity",
        "description": "View your basic identity info",
    },
    "email": {
        "header": "Email",
        "description": "View your email address",
    },
    "phone": {
        "header": "Phone",
        "description": "View your phone number",
    },
    "address": {
        "header": "Address",
        "description": "View your address",
    }
}

// def header
const header = computed(() => {
  try{
    return claims[props.claim].header
  }
  catch{
    throw new Error("Invalid claim")
  }
})
// def description
const description = computed(() => {
  try{
    return claims[props.claim].description
  }
  catch{
    throw new Error("Invalid claim")
  }
})



</script>


<template>
  <div class="flex flex-col justify-start b-1 b-dark-1 rounded-8px w-100% p-10px">
    <h1 class="text-2xl font-bold font-100 flex flex-col justify-center items-start ">{{ header }}</h1>
    <p class="text-lg font-100 flex flex-col justify-start items-start ">{{ description }}</p>
  </div>
</template>

