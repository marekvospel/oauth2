<script setup lang="ts">
const emit = defineEmits(['login'])

const loginData = reactive({
  email: '',
  password: '',
})

async function login(): Promise<void> {
  try {
    const result = await fetch('/api/auth/login', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        ...loginData,
      }),
    })

    if (result.ok)
      emit('login')
  }
  catch { }
}
</script>

<template>
  <div class="flex flex-row w-full h-screen">
    <div class="w-1/2 flex flex-col items-center justify-center">
      <h1 class="text-5xl flex flex-col justify-center items-center">
        Welcome back!
      </h1>
    </div>
    <div class="w-1/2">
      <form class="flex flex-col gap-2 max-w-100" @submit.prevent="login">
        <input v-model="loginData.email" class="bg-gray-300">
        <input v-model="loginData.password" class="bg-gray-300">
        <button type="submit">
          Sign in
        </button>
      </form>
    </div>
  </div>
</template>
