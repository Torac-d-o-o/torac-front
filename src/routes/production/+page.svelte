<script lang="ts">
    import { MixerStatus, ProductionStatus, type Order, type Production } from "$lib/types"
    import type { Mixer } from "$lib/types/Mixer"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { onMount } from "svelte"
    import MixerComponent from "./mixer_component.svelte"
    import { mixerMachineId } from "$lib/stores/mixer_machine_id.store"

    let ordersInWashing: Order[] | null = null;
    let selectedOrderId: number | null = null;
    let selectedOrder: Order | null = null;
    let selectedMixerId: number;
    $: selectedMixerId = $mixerMachineId;

    const token = getCookieAttribute('token');

    onMount(async () => {
        await getOrdersInWashing();
    });

    mixerMachineId.subscribe(value => {
        console.log("mixerMachineId has changed in +page.svelte:", value);
        selectedMixerId = value;
    });

    async function getOrdersInWashing() {
        try {
            const data = await invoke('get_orders', { token: token, status: 'IN_WASHING' });
            if (data) {
                ordersInWashing = JSON.parse(data as string);
            }
        } catch (error) {
            console.error("Error fetching orders in washing:", error);
        }
    }

    function selectOrder(order: Order) {
        selectedOrderId = order.id;
        selectedOrder = order;
    }
</script>

<div class="table-container px-5 h-1/4 mt-5">
    <table class="table table-hover table-interactive">
        <thead>
            <tr class="text-l my-2">
                <th class="flex justify-center">Order in Washing</th>
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
            {#if ordersInWashing}
                {#each ordersInWashing as order}
                    <tr on:click={() => selectOrder(order)} class="{selectedOrderId === order.id ? 'table-row-checked' : ''}">
                        <td>{order.id}</td>
                        <td>{order.boxIds}</td>
                        <td>{order.oliveAmount}</td>
                        <td>{formatDateTime(order.receivedAt.toString())}</td>
                        <td>{order.oilFiltering}</td>
                        <td>{order.oilWaterSeparation}</td>
                        <td>{order.status}</td>
                        <td>{order.bottleDescription}</td>
                    </tr>
                {/each}
            {:else}
                <tr>
                    <td colspan="8">No orders in washing</td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>

<div class="grid grid-cols-3 gap-4 pb-5 px-5 h-3/4">
    <MixerComponent id={1} {selectedOrderId}></MixerComponent>
    <MixerComponent id={2} {selectedOrderId}></MixerComponent>
    <MixerComponent id={3} {selectedOrderId}></MixerComponent>
    <MixerComponent id={4} {selectedOrderId}></MixerComponent>
    <MixerComponent id={5} {selectedOrderId}></MixerComponent>
    <MixerComponent id={6} {selectedOrderId}></MixerComponent>
</div>
