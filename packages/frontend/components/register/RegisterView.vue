<script setup lang="ts">
const emit = defineEmits(['register'])

const registerData = reactive({
  email: '',
  password: '',
  passwordRepeat: '',
})

async function check(): Promise<void> {
  if (registerData.password === registerData.passwordRepeat)
    await register()
}

async function register(): Promise<void> {
  try {
    const result = await fetch('/api/auth/register', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        ...registerData,
      }),
    })

    if (result.ok)
      emit('register')
  }
  catch { }
}
</script>

<template>
  <div class="flex flex-row w-full h-screen">
    <div class="w-1/2 flex flex-col items-center justify-center">
      <h1 class="text-6xl font-bold">
        Welcome to our platform!
      </h1>
    </div>
    <div class="w-1/2 flex flex-col justify-center items-center">
      <form class="flex flex-col gap-2 w-100" @submit.prevent="check">
        <BasicInput v-model="registerData.email" label="email" placeholder="john.doe@example.com" />
        <BasicInput v-model="registerData.password" type="password" label="password" placeholder="$3cr3tPa$$w0rd" />
        <BasicInput v-model="registerData.passwordRepeat" type="password" label="repeat password" placeholder="r3p3a7 $3cr3tPa$$w0rd" />
        <BasicButton text="Register" />
        <!-- <button type="submit">
          Sign in
        </button> -->
      </form>
    </div>
  </div>
</template>
