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
      <h1 class="text-6xl font-bold">
        Welcome back!
      </h1>
    </div>
    <div class="w-1/2 flex flex-col justify-center items-center">
      <form class="flex flex-col gap-2 w-100" @submit.prevent="login">
        <BasicInput v-model="loginData.email" label="email" placeholder="john.doe@example.com" />
        <BasicInput v-model="loginData.password" type="password" label="password" placeholder="$3cr3tPa$$w0rd" />
        <BasicButton text="Sign in" />
        <!-- <button type="submit">
          Sign in
        </button> -->
      </form>
    </div>
  </div>
</template>
