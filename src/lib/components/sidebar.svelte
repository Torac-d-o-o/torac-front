<script lang="ts">
import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton'
import { page } from '$app/stores'

import type { BasicRouteData } from '$lib/types'

export let routes: BasicRouteData[] = []

const checkCurrentPath = (route: string) => $page.url.pathname === route
</script>

<AppRail id="sidebar">
  {#each routes as page }
    <AppRailAnchor href={ page.route } selected={checkCurrentPath(page.route)}>
      <svelte:fragment slot="lead">
        <svelte:component this={page.icon} />
      </svelte:fragment>

      <span>{ page.name }</span>
    </AppRailAnchor>
  {/each}
</AppRail>
