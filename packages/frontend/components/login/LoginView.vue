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
  <div class="flex flex-col gap-8 justify-center md:flex-row w-full h-screen">
    <div class="md:w-1/2 flex flex-col items-center justify-center">
      <h1 class="text-5xl lg:text-6xl font-bold text-center">
        Welcome back!
      </h1>
    </div>
    <div class="md:w-1/2 flex flex-col justify-center items-center">
      <div class="xl:w-120 lg:w-80 w-full flex shrink-0">
        <form class="flex flex-col gap-2 w-full" @submit.prevent="login">
          <BasicInput v-model="loginData.email" label="email" placeholder="john.doe@example.com" />
          <BasicInput v-model="loginData.password" type="password" label="password" placeholder="$3cr3tPa$$w0rd" />
          <a href="/register" class="text-primary w-max">Register</a>
          <BasicButton text="Sign in" />
          <!-- <button type="submit">
          Sign in
        </button> -->
        </form>
      </div>
    </div>
  </div>
</template>
