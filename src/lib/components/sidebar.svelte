<script lang="ts">
    import { AppRail, AppRailAnchor, LightSwitch } from '@skeletonlabs/skeleton'
    import { page } from '$app/stores'

    import type { BasicRouteData } from '$lib/types'

    export let routes: BasicRouteData[] = []

    $: checkCurrentPath = (route: string) => $page.url.pathname === route
</script>

<AppRail id="sidebar">
    {#each routes as page}
        <AppRailAnchor href={page.route} selected={checkCurrentPath(page.route)}>
            <svelte:fragment slot="lead">
                <svelte:component
                    this={page.icon}
                    fill={checkCurrentPath(page.route) ? 'fill-black' : null}
                    height="36"
                    width="36"
                />
            </svelte:fragment>

            <span>{page.name}</span>
        </AppRailAnchor>
    {/each}

    <svelte:fragment slot="trail">
        <span class="grid place-content-center place-items-center mb-4">
            <LightSwitch />
        </span>
    </svelte:fragment>
</AppRail>
