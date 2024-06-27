<script lang="ts">
    import '../app.css'
    import HomeIcon from '$lib/components/icons/home.svelte'
    import ReceiptIcon from '$lib/components/icons/receipt.svelte'
    import Sidebar from '$lib/components/sidebar.svelte'
    import GearIcon from '$lib/components/icons/gear.svelte'
    import WaterIcon from '$lib/components/icons/water.svelte'
    import CustomerIcon from '$lib/components/icons/customer.svelte'
    import ScaleIcon from '$lib/components/icons/scale.svelte'
    import type { BasicRouteData } from '$lib/types'
    import TreeBranch from '$lib/components/icons/tree_branch.svelte'

    const routes: BasicRouteData[] = [
        {
            name: 'Dashboard',
            route: '/',
            icon: HomeIcon
        },
        {
            name: 'Order',
            route: '/order',
            icon: ReceiptIcon
        },
        {
            name: 'Cleaning',
            route: '/cleaning',
            icon: TreeBranch
        },
        {
            name: 'Washing',
            route: '/washing',
            icon: WaterIcon
        },
        {
            name: 'Production',
            route: '/production',
            icon: GearIcon
        },
        {
            name: 'Weighing',
            route: '/weighing',
            icon: ScaleIcon
        },
        {
            name: 'Customer',
            route: '/customer',
            icon: CustomerIcon
        },
        {
            name: 'Log In',
            route: '/login',
            icon: CustomerIcon
        },
    ]
</script>

<div class="grid grid-cols-[auto_1fr] h-dvh w-dvh">
    <Sidebar {routes} />

    <section>
        <slot />
    </section>
</div>
