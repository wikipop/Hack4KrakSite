export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path.startsWith('/tasks')) {
    const toast = useToast()
    let description = 'Nieznany błąd'

    try {
      const { error } = await useApi('/event/status', {
        key: 'event-status',
        redirect: 'error',
      })

      if (error.value?.data) {
        const response = error.value.data as any
        switch (response.code) {
          case 403:
            description = 'Nie możesz otworzyć tej strony przed rozpocząciem wydarzenia'
            break
          case 410:
            description = 'Nie możesz otworzyć tej strony po zakończeniu wydarzenia'
        }

        toast.add({ title: `Błąd ${response.code}`, description: `${description}`, color: 'error' })
        return '/'
      }
    } catch {
      toast.add({ title: 'Błąd', description: `${description}`, color: 'error' })
      return '/'
    }
  }
})
