<script setup lang="ts">
const isMobileMenuOpen = ref(false)
function toggleMobileMenu() {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

// Hide navbar on button press
const router = useRouter()
watch(() => router.currentRoute.value, () => {
  isMobileMenuOpen.value = false
})

const items = [
  [
    {
      label: 'Ranking',
      to: '/leaderboard',
    },
    {
      label: 'Regulamin',
      to: '/rules',
    },
    {
      label: 'FAQ',
      to: '/faq',
    },
    {
      label: 'Zadania',
      to: '/tasks',
    },
    {
      label: 'Discord',
      to: 'https://discord.gg/ASPqckzEd8',
      target: '_blank',
    },
  ],
  [
    {
      slot: 'button',
      to: '/login',
    },
  ],
]
</script>

<template>
  <UContainer class="top-0 z-10 sticky max-w-full border-b-1 border-neutral-600 font-pixelify bg-(--ui-bg)">
    <div class="mx-auto flex items-center">
      <NuxtLink to="/" class="flex items-center space-x-2 py-3">
        <Logo class="size-10 text-black dark:text-white" />
        <h1 class="md:hidden text-2xl font-semibold">
          Hack4Krak
        </h1>
      </NuxtLink>

      <!-- Desktop Navigation -->
      <LazyUNavigationMenu
        :items="items" variant="link" class="hidden md:flex w-full"
        :ui="{ linkLabel: 'text-lg hover:underline underline-offset-5 text-(--ui-bg) dark:text-white' }"
        hydrate-on-media-query="(min-width: 768px)"
      >
        <template #button>
          <ElevatedButton message="START GRY" />
        </template>
      </LazyUNavigationMenu>

      <button class="md:hidden p-2 ml-auto cursor-pointer" aria-label="Toogle navbar" @click="toggleMobileMenu">
        <Icon :name="isMobileMenuOpen ? 'mdi:close' : 'mdi:hamburger-menu'" size="24" />
      </button>
    </div>

    <!-- Mobile Navigation -->
    <Transition
      enter-from-class="opacity-0 translate-x-[100%]"
      leave-to-class="opacity-0 translate-x-[100%]"
      enter-active-class="transition duration-200"
      leave-active-class="transition duration-200"
      hydrate-on-media-query="(max-width: 768px)"
    >
      <div v-if="isMobileMenuOpen" class="md:hidden h-screen [&>a]:text-5xl ">
        <USeparator class="mb-2" />
        <LazyUNavigationMenu
          :items="items"
          orientation="vertical"
          variant="link"
          class="w-full text-3xl"
          :ui="{ link: 'text-lg text-white text-black dark:text-white' }"
        >
          <template #button>
            <div class="items-center justify-center text-center w-full">
              <ElevatedButton message="START GRY" class="w-70 mt-2" />
            </div>
          </template>
        </LazyUNavigationMenu>
      </div>
    </Transition>
  </UContainer>
</template>
