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
  <div class="flex flex-col gap-8 justify-center md:flex-row w-full h-screen">
    <div class="md:w-1/2 flex flex-col items-center justify-center">
      <h1 class="text-5xl lg:text-6xl font-bold text-center">
        Welcome to our platform!
      </h1>
    </div>
    <div class="md:w-1/2 flex flex-col justify-center items-center">
      <div class="xl:w-120 lg:w-80 w-full flex shrink-0">
        <form class="flex flex-col gap-2 w-full" @submit.prevent="check">
          <BasicInput v-model="registerData.email" label="email" placeholder="john.doe@example.com" />
          <BasicInput v-model="registerData.password" type="password" label="password" placeholder="$3cr3tPa$$w0rd" />
          <BasicInput v-model="registerData.passwordRepeat" type="password" label="repeat password" placeholder="$3cr3tPa$$w0rd" />
          <BasicButton text="Register" />
          <!-- <button type="submit">
                    Sign in
                </button> -->
        </form>
      </div>
    </div>
  </div>
</template>
