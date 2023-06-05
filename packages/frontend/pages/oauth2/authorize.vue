<script setup lang="ts">
import { useRouteQuery } from '@vueuse/router'

const router = useRouter()
const route = useRoute()

const { pending, error } = useMe()

const responseType = useRouteQuery('response_type')
const clientId = useRouteQuery('client_id')
const scope = computed(() => useRouteQuery('scope', '').value.split(/\s+/))
const reditectUri = useRouteQuery('redirect_uri')

const validScopes = ['identity']

const validResponseType = computed(() => responseType.value === 'code' || responseType.value === 'token')
const validScope = computed(() => scope.value.every(s => validScopes.includes(s.toLocaleLowerCase())))
// TODO: check client id
const validClient = computed(() => !isNaN(Number(clientId.value)))
const validRedirect = computed(() => {
  try {
    // eslint-disable-next-line no-new
    new URL(reditectUri.value?.toString() ?? '')
    return true
  }
  catch {
    return false
  }
})

const hi = ref()

async function authorize(): Promise<void> {
  const result = await fetch('/api/oauth2/authorize', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      response_type: responseType.value,
      client_id: Number(clientId.value),
      scope: scope.value.join(' '),
      redirect_uri: reditectUri.value,
      state: route.query.state,
    }),
  })
  const data = await result.json()
  hi.value = data

  if (result.ok && data.authorization_code) {
    const errorUri = new URL(reditectUri.value?.toString() ?? '')
    const search = new URLSearchParams(errorUri.search)
    search.set('code', data.authorization_code)
    if (route.query.state)
      search.set('state', route.query.state?.toString())
    errorUri.search = search.toString()

    window.location.href = errorUri.toString()
  }

  if (result.ok && data.access_token) {
    const errorUri = new URL(reditectUri.value?.toString() ?? '')
    const search = new URLSearchParams(errorUri.hash)
    search.set('access_token', data.access_token)
    if (data.refresh_token)
      search.set('refresh_token', data.refresh_token)
    search.set('token_type', data.token_type)
    search.set('expires_in', data.expires_in)
    search.set('scope', data.scope)
    if (route.query.state)
      search.set('state', route.query.state?.toString())
    errorUri.hash = search.toString()

    window.location.href = errorUri.toString()
  }
}

function cancel() {
  const errorUri = new URL(reditectUri.value?.toString() ?? '')
  const search = new URLSearchParams(errorUri.search)
  search.set('error', 'user_cancelled')
  errorUri.search = search.toString()

  window.location.href = errorUri.toString()
}

watchEffect(() => {
  if (!pending.value && error.value)
    router.replace(`/login/?redirect_to=${encodeURIComponent(route.fullPath)}`)
})
</script>

<template>
  <div>
    <AuthorizeError v-if="!validResponseType" error="Invalid response type" />
    <AuthorizeError v-else-if="!validScope" error="Invalid scope" />
    <AuthorizeError v-else-if="!validClient" />
    <AuthorizeError v-else-if="!validRedirect" error="Invalid redirect" />
    <div v-else-if="pending">
      Loading...
    </div>
    <form v-else @submit.prevent="authorize">
      <div class="flex flex-row w-full min-h-screen h-fit">
        <div class="w-200 sm:max-w-[80%] h-fit m-auto flex flex-col items-center border border-dark-300 rounded-lg p-3 w-[90%] sm:p-10">
          <h1 class="text-2xl font-bold font-100 flex flex-col justify-center items-center mb-2em ">
            Authorize {{ 'applicationName' }} access to your:
          </h1>
          <div class="flex flex-col justify-start items-start w-full gap-2 max-h-[70%] overflow-auto">
            <AuthorizeClaim v-for="scopeItem in scope" :scope="scopeItem as any" :key="scopeItem" />
          </div>
          <div class="flex flex-row flex-wrap gap-8 w-full justify-center mt-12">
            <div class="flex sm:w-50 w-full">
              <BasicButton variant="outline" @click.prevent="cancel">Cancel</BasicButton>
            </div>
            <div class="flex sm:w-50 w-full">
              <BasicButton type="submit">Authorize</BasicButton>
            </div>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>
