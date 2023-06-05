<script setup lang="ts">



interface Props {
  scope: keyof typeof claims,
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
  console.log(props.scope);
    return claims[props.scope]?.header
})
// def description
const description = computed(() => {
  return claims[props.scope]?.description
})



</script>


<template>
  <div class="flex flex-col justify-start border border-dark-1 rounded-lg w-full p-4">
    <h1 class="text-2xl font-bold justify-center items-start ">{{ header }}</h1>
    <p class="text-lg font-hairline flex flex-col justify-start items-start ">{{ description }}</p>
  </div>
</template>

