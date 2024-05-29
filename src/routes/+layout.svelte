<script lang='ts'>
import '../app.css'
import Sidebar from '$lib/components/sidebar.svelte'

import type { BasicRouteData } from '$lib/types'
import Production from '$lib/components/icons/production.svelte'
import Statistic from '$lib/components/icons/statistic.svelte'
import Reception from '$lib/components/icons/reception.svelte'
import Package from '$lib/components/icons/package.svelte'
import Scale from '$lib/components/icons/scale.svelte'
import TreeBranch from '$lib/components/icons/tree_branch.svelte'
import LogOut from '$lib/components/icons/log_out.svelte'

const routes: BasicRouteData[] = [
  {
    name: 'Reception',
    route: '/reception',
    icon: Reception
  },
  {
    name: 'Cleaning',
    route: '/cleaning',
    icon: TreeBranch
  },
  {
    name: 'Production',
    route: '/production',
    icon: Production
  },
  {
    name: 'Weighing',
    route: '/weighing',
    icon: Scale
  },
  {
    name: 'Oil Delivery',
    route: '/oil_delivery',
    icon: Package
  },
  {
    name: 'Statistic',
    route: '/statistic',
    icon: Statistic
  },
  {
    name: 'Log out',
    route: '/',
    icon: LogOut
  }
]
</script>

<div class="grid grid-cols-[auto_1fr] h-dvh w-dvh">
  <Sidebar routes={routes} />

  <section>
    <slot />
  </section>
</div>
