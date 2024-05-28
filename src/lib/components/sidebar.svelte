<script lang="ts">
import { AppRail, AppRailAnchor, LightSwitch } from '@skeletonlabs/skeleton'
import { page } from '$app/stores'

import type { BasicRouteData } from '$lib/types'

export let routes: BasicRouteData[] = []

const checkCurrentPath = (route: string) => $page.url.pathname === route
</script>

<AppRail id="sidebar" class="w-32">
  {#each routes as page }
    <AppRailAnchor href={ page.route } selected={checkCurrentPath(page.route)}>
      <svelte:fragment slot="lead">
        <svelte:component this={page.icon} />
      </svelte:fragment>

      <span class="text-xl">{ page.name }</span>
    </AppRailAnchor>
  {/each}

  <svelte:fragment slot="trail">
    <span class="grid place-content-center place-items-center mb-4">
      <LightSwitch />
    </span>
  </svelte:fragment>
</AppRail>
