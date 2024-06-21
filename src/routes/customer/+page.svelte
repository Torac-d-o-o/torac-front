<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import type { PageData } from './$types'
    import data from '../order/+page.svelte'
    import { onMount } from "svelte"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"


    const token = getCookieAttribute('token');
        onMount(() => {
            console.log(token);
    })

</script>


<div class="flex justify-center">Hello</div>