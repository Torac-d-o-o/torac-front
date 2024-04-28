<script lang='ts'>
import '../../app.css'
import Sidebar from '$lib/components/sidebar.svelte'
import type { BasicRouteData } from '$lib/types'

const routes: BasicRouteData[] = [
  {
    name: 'Home',
    route: '/home'
  },
  {
    name: 'Reception',
    route: '/order_reception'
  },
  {
    name: 'Olive Cleaning',
    route: '/olive_cleaning',
  },
  {
    name: 'Production',
    route: '/order_production',
  },
  {
    name: 'Weighing',
    route: '/order_weighing',
  },
  {
    name: 'Filtering',
    route: '/order_filtering',
  },
  {
    name: 'Statistics',
    route: '/production_statistic_data',
  },
  {
    name: 'Log out',
    route: '/',
  },
]
</script>

<slot />

<Sidebar routes={routes} />
