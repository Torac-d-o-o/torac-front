<script lang="ts">
    import '../app.css'
    import HomeIcon from '$lib/components/icons/home.svelte'
    import Sidebar from '$lib/components/sidebar.svelte'

    import type { BasicRouteData } from '$lib/types'

    const routes: BasicRouteData[] = [
        {
            name: 'Dashboard',
            route: '/',
            icon: HomeIcon
        }
    ]
</script>

<div class="grid grid-cols-[auto_1fr] h-dvh w-dvh">
    <Sidebar {routes} />

    <section>
        <slot />
    </section>
</div>
