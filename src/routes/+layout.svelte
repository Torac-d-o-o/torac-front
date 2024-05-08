<script lang='ts'>
import '../app.css'
import Sidebar from '$lib/components/sidebar.svelte'
import HomeIcon from '$lib/components/icons/home.svelte'

import type { BasicRouteData } from '$lib/types'

const routes: BasicRouteData[] = [
  {
    name: 'Home',
    route: '/',
    icon: HomeIcon
  },
]
</script>

<div class="grid grid-cols-[auto_1fr] h-dvh w-dvh">
  <Sidebar routes={routes} />

  <section>
    <slot />
  </section>
</div>
