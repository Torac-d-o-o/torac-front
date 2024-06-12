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
        await invoke('get_orders', { token: token , status: 'RECEIVED' }).then((data) => {
            orders = JSON.parse(data as string)
        })
    }

    //onMount(() => { getOrders()
    //    console.log(orders);
    // })

</script>

<section class="flex justify-center mb-10">
    <div >
        <button on:click={getOrders}>get orders</button>
    </div>
</section>

<div class="table-container">
    <table class="table table-hover">
        <thead>
            <tr>
                <td>ID</td>
                <td>Box ID</td>
                <td>Olive Amount</td>
                <td>Received At</td>
                <td>Filtering Option</td>
                <td>Separation Option</td>
                <td>Status</td>
                <td>Bottle Description</td>
            </tr>
        </thead>
        <tbody class="grid grid-cols-1">
        {#if orders != null}
        {#each orders as single_order}
        <tr class="grid grid-cols-8">
            <td>{single_order.id}</td>
            <td>{single_order.boxIds}</td>
            <td>{single_order.oliveAmount}</td>
            <td>{single_order.receivedAt}</td>
            <td>{single_order.oilFiltering}</td>
            <td>{single_order.oilWaterSeparation}</td>
            <td>{single_order.status}</td>
            <td>{single_order.bottleDescription}</td>
        </tr>
        {/each}
        {:else}
             <div>Nothing there yet</div>
        {/if}
        </tbody>
    </table>
</div>