<script setup lang="ts">

let emit = defineEmits(['login'])

const loginData = reactive({
  email: '',
  password: '',
})

async function login(): Promise<void> {

  try {
    let result = await fetch('/api/auth/login', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        ...loginData
      })
    })

    if (result.ok) {
      emit('login')
    }
  } catch {}
}

</script>

<template>
  <form @submit.prevent="login" class="flex flex-col gap-2 max-w-100">
    <input class="bg-gray-300" v-model="loginData.email"/>
    <input class="bg-gray-300" v-model="loginData.password"/>
    <button type="submit">Sign in</button>
  </form>
</template>