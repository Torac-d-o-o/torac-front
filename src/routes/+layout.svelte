<script lang='ts'>
import '../app.css'
import Sidebar from '$lib/components/sidebar.svelte'
import HomeIcon from '$lib/components/icons/home.svelte'

import type { BasicRouteData } from '$lib/types'
import Production from '$lib/components/icons/production.svelte'
  import Statistic from '$lib/components/icons/statistic.svelte'
  import Reception from '$lib/components/icons/reception.svelte'
  import Package from '$lib/components/icons/package.svelte'

const routes: BasicRouteData[] = [
  {
    name: 'Home',
    route: '/',
    icon: HomeIcon
  },
  {
    name: 'Production',
    route: '/production',
    icon: Production
  },
  {
    name: 'Statistic',
    route: '/statistic',
    icon: Statistic
  },
  {
    name: 'Reception',
    route: '/reception',
    icon: Reception
  },
  {
    name: 'Oil Delivery',
    route: '/oil_delivery',
    icon: Package
  }
]
</script>

<div class="grid grid-cols-[auto_1fr] h-dvh w-dvh">
  <Sidebar routes={routes} />

  <section>
    <slot />
  </section>
</div>
