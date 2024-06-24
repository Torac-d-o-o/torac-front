<script lang="ts">
    import { onMount } from "svelte"
    import { mixerMachineId } from "$lib/stores/mixer_machine_id.store"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { MixerMachines, MixerStatus, ProductionStatus, type Mixer, type Production } from "$lib/types"

    export let id: string | MixerMachines
    export let selectedOrderId: number | null
    const token = getCookieAttribute('token')
    let foundMixer: Mixer | null = null
    let productionFromOrder: Production | null = null

    onMount(async () => {
        console.log("Mixer id: ", id)
        await getMixerData()
    });


    async function getMixerData() {
        await invoke('get_mixers', { token: token, data: { machineId: Number(id), mixerStatus: MixerStatus.FULL_MIXER } })
            .then((data) => {
                if (data !== null) {
                    foundMixer = JSON.parse(data as string);
                }
            });
    }

    async function getProductionByOrder(orderId: number) {
        if (orderId !== null) {
            await invoke('get_production', { token: token, order: orderId, status: ProductionStatus.IN_PRODUCTION })
            .then((data) => {
                if (data !== null) {
                    productionFromOrder = JSON.parse(data as string)
                }
            })         
        }
    }

    async function registerMixer() {
        if (selectedOrderId !== null) {
            await getProductionByOrder(Number(selectedOrderId))
            if (productionFromOrder) {
                const response = await invoke('register_mixer', { token: token, data: { machineId: Number(id), production: productionFromOrder.id }})
                if (response) {
                    await getMixerData()
                }
            }
        }
    }

    async function sendToDecanter() {
        if (id && foundMixer?.production.id) {
            await invoke('register_decanter', { token: token, data: { production: foundMixer.production.id, mixer: id.toString() }})
            .then((data) => {
                if (data !== null) {
                    invoke('update_mixer', { machineId: foundMixer?.machineId, status: MixerStatus.EMPTY_MIXER, token: token })
                    .then(() => {
                        foundMixer = null
                    })
                }
            })
        }
    }
</script>

<div class="card p-4">
    {#if id == foundMixer?.machineId}
        {#key foundMixer}
        <label for="" class="flex justify-center text-2xl py-4">
            Mixer {foundMixer.machineId}
        </label>
        <label for="">
            Entered At: {formatDateTime(foundMixer.enteredAt.toString())}
        </label>
        <label for="">
            Production ID: {foundMixer.production.id}
        </label>
        <label for="">
            Approximated Pasta Weight: {foundMixer.approximatedPastaWeight}
        </label>
        <label for="">
            Order ID: {foundMixer.production.order.id}
        </label>
        <label for="">
            Customer Name: {foundMixer.production.order.customer.nameSurname}
        </label>
        <button type="button" class="btn variant-filled" on:click={sendToDecanter}>Send to Decanter</button>
        {/key}
    {:else}
        <label for="" class="flex justify-center text-2xl pb-4">
            Mixer {id}
        </label>
        <label for="">
            EMPTY
        </label>
        <button type="submit" class="btn variant-filled" on:click={registerMixer}>Enter Mixer</button>
    {/if}
</div>
