<script lang="ts">
    import { type Order } from "$lib/types"
    import type { Mixer } from "$lib/types/Mixer"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { dataDir } from "@tauri-apps/api/path"
    import { onMount } from "svelte"
    import MixerComponent from "./mixer_component.svelte"
    import { mixerMachineId } from "$lib/stores/mixer_machine_id.store"

    
    let ordersInWashing: Order[] | null = null
    let selectedOrderId: number
    let selectedMixerId: number
    $: selectedMixerId = $mixerMachineId
    let loadedMixers: Mixer[] | null = null
    
    const token = getCookieAttribute('token');

    onMount(() => {
        getOrdersInWashing()
    })

    mixerMachineId.subscribe(value => {
        console.log("mixerMachineId has changed in AnotherComponent:", value)
        selectedMixerId = value
    });

    async function getOrdersInWashing() {
        await invoke('get_orders', { token: token, status: 'IN_WASHING' })
        .then((data) => {
            if (data) {
                ordersInWashing = JSON.parse(data as string)
            }
        })
    }

    function selectOrder(orderId: number) {
        selectedOrderId = orderId
    }

    async function getMixers() {
        await invoke('get_mixers', {token: token})
        .then((data) => {
            loadedMixers = JSON.parse(data as string)
        })
    }

    async function registerMixer() {
        await invoke('register_mixer', { token: token, data: { }})
        .then((data) => {
            console.log(data)
        })
    }

    async function sendToDecanter() {
        if (selectedMixerId) {
            await invoke('register_decanter', { token: token, data: {  }})
            .then((data) => {
                if (data !== null) {
                    console.log(data)
                }
            })
        }
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
            {#if ordersInWashing !== null}
                {#each ordersInWashing as order}
                    <tr on:click={() => selectOrder(order.id)} class="{selectedOrderId===order.id ? 'table-row-checked' : ''}">
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

<div class="grid grid-cols-3 gap-4 pb-5 px-5 h-1/2">
    <MixerComponent id=1></MixerComponent>
    <MixerComponent id=2></MixerComponent>
    <MixerComponent id=3></MixerComponent>
    <MixerComponent id=4></MixerComponent>
    <MixerComponent id=5></MixerComponent>
    <MixerComponent id=6></MixerComponent>
</div>


<div class="flex place-content-center" on:submit={registerMixer}>
    <form action="" class="grid grid-cols-2 gap-8 place-self-center">
        <div>
            <label class="label" for="">Selected Order ID:</label>
            <input type="number" class="input" value="{selectedOrderId}">
        </div>
        <div>
            <label class="label" for="">Selected Mixer ID:</label>
            <input type="number" class="input" value="{selectedMixerId}">
        </div>
            <button type="submit" class="btn variant-filled">Enter Mixer</button>
        <div>
            <button type="button" class="btn variant-filled">Send to Decanter and Separator</button>
        </div>
    </form>
</div>