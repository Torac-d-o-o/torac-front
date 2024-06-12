<script lang="ts">
    import { OrderStatus, type Order } from "$lib/types"
    import type { Mixer } from "$lib/types/Mixer"
    import type { Production } from "$lib/types/Production"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { onMount } from "svelte"

    let selectedOrderId: number | null = null
    let status: OrderStatus = OrderStatus.IN_WASHING
    let orderInWashing: Order[]
    let token: string = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.pu_0ulHSFflnkiwT09tHq2nIsViM8XYIqBsxe1hs-qKZ-qU-mVbjBdfNsgkNvQ-WxusKXGhy79MWdMRTVi2sCA"
    let mixers: Mixer[] | null
    let selectedMixerId: number | null = null
    let selectedMixer: Mixer | null = null
    let productionsOfSelectedOrder: Production[]

    async function getWashingOrder() {
        console.log("before invoke"); 
        await invoke('get_orders', { token: token, status: status }).then((data) => {
            orderInWashing = JSON.parse(data as string);
            console.log("Orders in washing:", orderInWashing);
        }).catch(err => console.error("Error fetching orders in cleaning:", err));
    }

    async function getProductionFromOrderId(order: number) {
        console.log("Selected order id:", order)
        await invoke('get_production', { token: token, status: 'IN_PRODUCTION', order: order })
        .then((data) => {
            console.log("Data after get_production request", data)
            productionsOfSelectedOrder = JSON.parse(data as string)
        }).catch(err => console.error("Error fetching production from order ID:", err))
        console.log("Productions from selected order id:", productionsOfSelectedOrder)
    }

    function selectOrder(orderId: number) {
        selectedOrderId = orderId
        getProductionFromOrderId(orderId)
    }

    function selectMixer(mixerId: number) {
        selectedMixerId = mixerId
        console.log("Selected mixer: ", selectedMixerId)
    }

    function selectMixerFromFull(mixerId: number, mixer: Mixer) {
        selectedMixerId = mixerId
        console.log("Selected mixer: ", selectedMixerId)
        selectedMixer = mixer
    }

    async function getMixers() {
        await invoke('get_mixers', {token: token, data: {status: OrderStatus.IN_PRODUCTION} })
        .then((data) => {
            if (data !== null) orderInWashing = JSON.parse(data as string)
        })
    }

    async function registerMixer() {
        console.log(selectedMixerId)
        console.log(selectedOrderId)
        
        if (selectedMixerId && selectedOrderId) {
            await invoke('register_mixer', {
                token: token,
                data: {
                    machineId: selectedMixerId,
                    orderId: selectedOrderId,
                    productionId: productionsOfSelectedOrder[0].id
                }
            }).then((data) => {
                console.log(JSON.parse(data as string))
                if (!data) {
                    console.log("ERROR REGISTER MIXER");
                }
            })
            .then(() => { getMixers() });
        }
    }

    async function sendToDecanter() {
        
    }

    onMount(() => {
        console.log("Mounting");
        getWashingOrder()
        .then(() => {getMixers()})
        console.log("after washing order");
    })
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
            {#if orderInWashing}
                {#each orderInWashing as order}
                    <tr on:click={() => selectOrder(order.id)} class="{selectedOrderId === order.id ? 'table-row-checked' : ''}">
                        <td>{order.id}</td>
                        <td>{order.boxIds}</td>
                        <td>{order.oliveAmount}</td>
                        <td>{order.receivedAt}</td>
                        <td>{order.oilFiltering}</td>
                        <td>{order.oilWaterSeparation}</td>
                        <td>{order.status}</td>
                        <td>{order.bottleDescription}</td>
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

<div class="grid grid-cols-3 grid-rows-2 px-4">
    {#if mixers != null }
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        {#each mixers.slice(0, 6) as mixer, index}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div on:click={() => selectMixerFromFull(mixer.id, mixer)} class="{selectedMixerId === mixer.id ? 'table-row-checked card hover:cursor-pointer' : 'card hover:cursor-pointer'}">
                <header class="card-header">Mixer {mixer.machineId}</header>
                <section class="p-4">
                    <label class="label" for="">Mixer data ID: {mixer.id}</label>
                    <label class="label" for="">Entered At: {mixer.enteredAt}</label>
                    <label class="label" for="">Approximated Weight Inside: {mixer.approximatedPastaWeight}</label>
                    <label class="label" for="">Production ID: {mixer.productionId}</label>
                </section>
                <footer class="card-footer">
                    <button class="btn">More Details</button>
                </footer>
            </div>
        {/each}
        {#if mixers.length < 6}
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            {#each Array.from({ length: 6 - mixers.length }, (_, i) => i) as i}
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div on:click={() => selectMixer(i)} class="{selectedMixerId === i ? 'table-row-checked card hover:cursor-pointer empty-mixer' : 'card hover:cursor-pointer empty-mixer'}">
                    <header class="card-header">Empty Mixer</header>
                    <section class="p-4">
                        <label class="label" for="">No data available</label>
                    </section>
                </div>
            {/each}
        {/if}
    {:else}
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        {#each Array.from({ length: 6 }, (_, i) => i + 1) as index}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div on:click={() => selectMixer(index)} class="{selectedMixerId === index ? 'table-row-checked card hover:cursor-pointer empty-mixer' : 'card hover:cursor-pointer empty-mixer'}">
                <header class="card-header">Empty Mixer</header>
                <section class="p-4">
                    <label class="label" for="">No data available</label>
                </section>
            </div>
        {/each}
    {/if}
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


<style>
    .empty-mixer {
        border: 1px solid #ccc;
        padding: 8px;
        margin: 8px;
        text-align: center;
    }
</style>