interface User {
  id: number
  username: string
  email: string
  error?: undefined
}

export function useMe() {
  return useFetch<User>('/api/@me', {
    method: 'GET',
    server: false,
  })
}
