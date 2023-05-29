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
  catch {}
}
</script>

<template>
  <form class="flex flex-col gap-2 max-w-100" @submit.prevent="login">
    <input v-model="loginData.email" class="bg-gray-300">
    <input v-model="loginData.password" class="bg-gray-300">
    <button type="submit">
      Sign in
    </button>
  </form>
</template>
