<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import type { PageData } from './$types'
    import data from '../order/+page.svelte'
    import { onMount } from "svelte"

    let token = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.pu_0ulHSFflnkiwT09tHq2nIsViM8XYIqBsxe1hs-qKZ-qU-mVbjBdfNsgkNvQ-WxusKXGhy79MWdMRTVi2sCA"
    const status: OrderStatus = OrderStatus.RECEIVED

    let orders: Order[]

    async function getOrders() {
        await invoke('get_orders', { token: token }).then((data) => {
            orders = JSON.parse(data as string)
        })
    }

    //onMount(() => { getOrders()
    //    console.log(orders);
    // })

</script>


<div class="flex justify-center">Hello</div>
<section class="flex justify-center">
    <div >
        <button on:click={getOrders}>get orders</button>
    </div>
</section>