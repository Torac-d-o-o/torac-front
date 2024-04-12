<script lang='ts'>
import '../app.css'
import Sidebar from '$lib/components/sidebar.svelte'
import type { BasicRouteData } from '$lib/types'

const routes: BasicRouteData[] = [
  {
    name: 'Home',
    route: '/'
  },
]
</script>

<slot />

<Sidebar routes={routes} />
