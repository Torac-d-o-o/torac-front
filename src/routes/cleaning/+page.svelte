<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import type { PageData } from './$types'
    import data from '../order/+page.svelte'
    import { onMount } from "svelte"


    let token = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.pu_0ulHSFflnkiwT09tHq2nIsViM8XYIqBsxe1hs-qKZ-qU-mVbjBdfNsgkNvQ-WxusKXGhy79MWdMRTVi2sCA"
    const status: OrderStatus = OrderStatus.RECEIVED

    let orders: Order[] | null = null

    async function getOrders() {
        await invoke('get_orders', { token: token }).then((data) => {
            orders = JSON.parse(data as string)
        })
    }

    //onMount(() => { getOrders()
    //    console.log(orders);
    // })

</script>

<section class="flex justify-center">
    <div >
        <button on:click={getOrders}>get orders</button>
    </div>
</section>


<div class="flex justify-center">
    <div class="grid grid-cols-1">
        {#if orders != null}
    {#each orders as single_order}
    <div class="grid grid-cols-8">
        <div>{single_order.id}</div>
        <div>{single_order.boxIds}</div>
        <div>{single_order.oliveAmount}</div>
        <div>{single_order.receivedAt}</div>
        <div>{single_order.oilFiltering}</div>
        <div>{single_order.oilWaterSeparation}</div>
        <div>{single_order.status}</div>
        <div>{single_order.bottleDescription}</div>
    </div>
    {/each}
    {:else}
         <div>Nothing there yet</div>
    {/if}
    </div>
</div>