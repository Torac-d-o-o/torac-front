<script lang="ts">
    import { onMount } from "svelte"
    import { mixerMachineId } from "$lib/stores/mixer_machine_id.store"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { MixerMachines, MixerStatus, type Mixer } from "$lib/types"

    export let id: string | MixerMachines
    const token = getCookieAttribute('token')
    let foundMixer: Mixer | null = null

    onMount(() => {
        console.log("Mixer id: ", id)
        getMixerData()
    });

    async function getMixerData() {
        await invoke('get_mixers', { token: token, data: { machineId: Number(id), mixerStatus: MixerStatus.FULL_MIXER } })
            .then((data) => {
                if (data !== null) {
                    console.log(data);
                    foundMixer = JSON.parse(data as string);
                    console.log(foundMixer);
                }
            });
    }

    function selectMixer() {
        mixerMachineId.set(Number(id));
        console.log("Set mixerMachineId to:", Number(id));
    }
</script>

<div class="card p-4">
    {#if id == foundMixer?.machineId}
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
        <button type="button" class="btn variant-filled" on:click={selectMixer}>Select Mixer</button>
    {:else}
        <label for="" class="flex justify-center text-2xl pb-4">
            Mixer {id}
        </label>
        <label for="">
            EMPTY
        </label>
        <button type="button" class="btn variant-filled" on:click={selectMixer}>Select Mixer</button>
    {/if}
</div>
