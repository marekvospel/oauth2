<script setup lang="ts">
import { useRouteQuery } from '@vueuse/router'

const route = useRoute()

const responseType = useRouteQuery('response_type')
const clientId = useRouteQuery('client_id')
const scope = computed(() => useRouteQuery('scope', '').value.split(/\s+/))
const reditectUri = useRouteQuery('redirect_uri')

const validScopes = ['identity']

const validResponseType = computed(() => responseType.value === 'code' || responseType.value === 'token')
const validScope = computed(() => scope.value.every((s) => validScopes.includes(s.toLocaleLowerCase())))
// TODO: check client id
const validClient = computed(() => !isNaN(Number(clientId.value)))
const validRedirect = computed(() => {
  try {
    new URL(reditectUri.value?.toString() ?? '')
    return true
  } catch {
    return false
  }
})

async function authorize(): Promise<void> {

  const result = await fetch('/api/oauth2/authorize', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      response_type: responseType.value,
      client_id: Number(clientId.value),
      scope: scope.value.join(' '),
      redirect_uri: reditectUri.value,
      state: route.query['state'],
    })
  })
  const data = await result.json()

  if (result.ok && data.authorization_code) {
    const errorUri = new URL(reditectUri.value?.toString() ?? '')
    const search = new URLSearchParams(errorUri.search)
    search.set('code', data.authorization_code)
    if (route.query['state']) search.set('state', route.query['state']?.toString())
    errorUri.search = search.toString()

    window.location.href = errorUri.toString()
  }

  if (result.ok && data.access_token) {
    const errorUri = new URL(reditectUri.value?.toString() ?? '')
    const search = new URLSearchParams(errorUri.hash)
    search.set('access_token', data.access_token)
    if (data.refresh_token) search.set('refresh_token', data.refresh_token)
    search.set('token_type', data.token_type)
    search.set('expires_in', data.expires_in)
    search.set('scope', data.scope)
    if (route.query['state']) search.set('state', route.query['state']?.toString())
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
</script>

<template>
  <div>
    <AuthorizeError v-if="!validResponseType" error="Invalid response type"/>
    <AuthorizeError v-else-if="!validScope" error="Invalid scope"/>
    <AuthorizeError v-else-if="!validClient" />
    <AuthorizeError v-else-if="!validRedirect" error="Invalid redirect" />
    <form v-else @submit.prevent="authorize">
      <p>An external application wants to access your account</p>
      <button @click="cancel" type="button">Cancel</button>
      <button class="bg-green" type="submit">Authorize</button>
    </form>
  </div>
</template>