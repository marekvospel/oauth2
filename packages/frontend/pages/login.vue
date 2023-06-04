<script setup lang="ts">
const router = useRouter()
const route = useRoute()
const redirectTo = computed(() => route.query?.redirect_to ?? '/')

const loginData = reactive({
  email: '',
  password: '',
})
const error = ref<undefined | 'credentials' | 'server'>()

async function login(): Promise<void> {
  error.value = undefined
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
      router.push(redirectTo.value.toString())
    else if (result.status >= 500 && result.status < 600)
      error.value = 'server'
    else
      error.value = 'credentials'
  }
  catch {
    error.value = 'server'
  }
}
</script>

<template>
  <div class="flex flex-col gap-8 justify-center md:flex-row w-full min-h-screen">
    <div class="md:w-1/2 flex flex-col items-center justify-center">
      <h1 class="text-5xl lg:text-6xl font-bold text-center">
        {{ $t('login.title') }}
      </h1>
    </div>
    <div class="md:w-1/2 flex flex-col justify-center items-center">
      <div class="xl:w-120 lg:w-80 w-full flex shrink-0">
        <form class="flex flex-col gap-2 w-full" @submit.prevent="login">
          <BasicInput v-model="loginData.email" type="email" label="email" placeholder="john.doe@example.com" />
          <BasicInput v-model="loginData.password" type="password" label="password" placeholder="$3cr3tPa$$w0rd" />
          <a href="/register" class="text-primary w-max">{{ $t('login.register') }}</a>
          <BasicButton type="submit">
            {{ $t('login.login') }}
          </BasicButton>
        </form>
      </div>
    </div>
  </div>
</template>
