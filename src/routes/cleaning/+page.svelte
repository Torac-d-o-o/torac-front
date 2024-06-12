<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import { onMount } from "svelte"
    import { formatDateTime } from "$lib/utils/date_formater"

    let token = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.pu_0ulHSFflnkiwT09tHq2nIsViM8XYIqBsxe1hs-qKZ-qU-mVbjBdfNsgkNvQ-WxusKXGhy79MWdMRTVi2sCA"

    let orders: Order[] | null = null
    let selectedOrderId: number | null = null
    let ordersInCleaning: Order[] | null = null

    async function getOrders() {
        console.log("Fetching orders...");
        await invoke('get_orders', { token: token, status: 'RECEIVED' }).then((data) => {
            orders = JSON.parse(data as string);
            console.log("Received orders:", orders);
        }).catch(err => console.error("Error fetching received orders:", err));
        
        await invoke('get_orders', { token: token, status: 'IN_CLEANING' }).then((data) => {
            ordersInCleaning = JSON.parse(data as string);
            console.log("Orders in cleaning:", ordersInCleaning);
        }).catch(err => console.error("Error fetching orders in cleaning:", err));
    }

    async function updateOrder(status: OrderStatus) {
        if (selectedOrderId !== null) {
            console.log(`Updating order ${selectedOrderId} to status ${status}`);
            await invoke('update_order', { orderId: selectedOrderId, status: status, token: token })
                .then(() => {
                    console.log("Order updated successfully");
                    getOrders();
                })
                .catch(err => console.error("Error updating order:", err));
        } else {
            console.warn("No order selected for update");
        }
    }

    onMount(() => { 
        getOrders();
    })

    function selectOrder(orderId: number) {
        selectedOrderId = orderId
    }
</script>

<div class="table-container px-5 h-1/2 mt-5">
    <table class="table table-hover table-interactive">
        <thead>
            <tr class="text-l my-2">
                <th class="flex justify-center">Received Orders</th>
            </tr>
            <tr>
                <th>ID</th>
                <th>Box ID</th>
                <th>Olive Amount</th>
                <th>Received At</th>
                <th>Filtering Option</th>
                <th>Separation Option</th>
                <th>Status</th>
                <th>Bottle Description</th>
            </tr>
        </thead>
        <tbody>
            {#if orders != null}
                {#each orders as single_order}
                <tr on:click={() => selectOrder(single_order.id)} class="{selectedOrderId===single_order.id ? 'table-row-checked' : ''}">
                    <td>{single_order.id}</td>
                    <td>{single_order.boxIds}</td>
                    <td>{single_order.oliveAmount}</td>
                    <td>{formatDateTime(single_order.receivedAt.toString())}</td>
                    <td>{single_order.oilFiltering}</td>
                    <td>{single_order.oilWaterSeparation}</td>
                    <td>{single_order.status}</td>
                    <td>{single_order.bottleDescription}</td>
                </tr>
                {/each}
            {:else}
                <tr>
                    <td colspan="8">Nothing there yet</td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
<div class="table-container px-5 h-1/2 mt-5">
    <table class="table table-hover table-interactive">
        <thead>
            <tr class="text-l my-2">
                <th class="flex justify-center">Orders In Cleaning</th>
            </tr>
            <tr>
                <th>ID</th>
                <th>Box ID</th>
                <th>Olive Amount</th>
                <th>Received At</th>
                <th>Filtering Option</th>
                <th>Separation Option</th>
                <th>Status</th>
                <th>Bottle Description</th>
            </tr>
        </thead>
        <tbody>
            {#if ordersInCleaning != null}
                {#each ordersInCleaning as cleaning_order}
                <tr on:click={() => selectOrder(cleaning_order.id)} class="{selectedOrderId===cleaning_order.id ? 'table-row-checked' : ''}">
                    <td>{cleaning_order.id}</td>
                    <td>{cleaning_order.boxIds}</td>
                    <td>{cleaning_order.oliveAmount}</td>
                    <td>{formatDateTime(cleaning_order.receivedAt.toString())}</td>
                    <td>{cleaning_order.oilFiltering}</td>
                    <td>{cleaning_order.oilWaterSeparation}</td>
                    <td>{cleaning_order.status}</td>
                    <td>{cleaning_order.bottleDescription}</td>
                </tr>
                {/each}
            {:else}
                <tr>
                    <td colspan="8">Nothing there yet</td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
<section class="grid grid-cols-4 px-5">
    <div>
        <button type="button" class="btn variant-filled" on:click={getOrders}>Refresh Orders</button>
    </div>
    <div>
        <button type="button" class="btn variant-filled" on:click={() => updateOrder(OrderStatus.IN_CLEANING)} disabled={selectedOrderId === null}>Start Cleaning Order</button>
    </div>
    <div>
        <button type="button" class="btn variant-filled" on:click={() => updateOrder(OrderStatus.IN_WASHING)} disabled={selectedOrderId === null}>Start Washing Order</button>
    </div>
    <div>
        <button type="button" class="btn variant-filled" on:click={() => updateOrder(OrderStatus.STORED_CLEAN)} disabled={selectedOrderId === null}>Stash Order</button>
    </div>
</section>
