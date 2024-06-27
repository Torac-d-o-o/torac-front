<script lang="ts">
    import { OrderStatus, type Order } from "$lib/types"
    import { ProductionStatus, type Production } from "$lib/types/Production"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { invoke } from "@tauri-apps/api"
    import { onMount } from "svelte"
    
    const token = window.localStorage.getItem("token")
    
    let enteredAt: Date | null = null
    let exitedAt: Date | null = null
    let status: ProductionStatus = ProductionStatus.IN_PRODUCTION
    let controlPanelUser: string = ''
    let oliveWashingUser: string = ''
    let oilWaterSeparationUser: string = ''
    const workerUsername = window.localStorage.getItem('username')
    let formError: string | null = null
    let selectedOrderId: number

    let stashedOrders: Order[] | null = null

    $: inputClasses = formError ? 'input input-error' : 'input'

    async function getStashedOrders() {
        await invoke('get_orders', { token: token, status: 'STORED_CLEAN'})
        .then((data) => {
            if (data !== null) {
                stashedOrders = JSON.parse(data as string)
            }
        })
    }

    function selectOrder(orderId: number) {
        selectedOrderId = orderId
    }

    onMount(() => {
        getStashedOrders()
    })

    const handleSubmit = async (event: SubmitEvent) => {
        // Stop the form event.
        event.preventDefault()

        const payload = {
            data: {
                enteredAt: enteredAt,
                exitedAt: exitedAt,
                controlPanelUser: controlPanelUser,
                oliveWashingUser: oliveWashingUser,
                oilWaterSeparationUser: oilWaterSeparationUser,
                status: status,
                workerUsername: workerUsername,
                order: selectedOrderId
            },
            token: token
        };

        try {
            const response: string | null = await invoke('register_production', payload)
            if (response === null) {
                formError = 'Failed to register production.';
                return;
            }
            await invoke('update_order', { token: token, status: OrderStatus.IN_WASHING, orderId: selectedOrderId}).then(() => getStashedOrders())
            console.log(JSON.parse(response));
        } catch (error) {
            console.error('Error registering production:', error);
            formError = 'Failed to register production.';
            }
        }
</script>

<div class="table-container px-5 h-1/2 mt-5">
    <table class="table table-hover table-interactive">
        <thead>
            <tr class="text-l my-2">
                <th class="flex justify-center">Clean Stashed Orders</th>
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
            {#if stashedOrders !== null}
                {#each stashedOrders as stashed_order}
                <tr on:click={() => selectOrder(stashed_order.id)} class="{selectedOrderId===stashed_order.id ? 'table-row-checked' : ''}">
                    <td>{stashed_order.id}</td>
                    <td>{stashed_order.boxIds}</td>
                    <td>{stashed_order.oliveAmount}</td>
                    <td>{formatDateTime(stashed_order.receivedAt.toString())}</td>
                    <td>{stashed_order.oilFiltering}</td>
                    <td>{stashed_order.oilWaterSeparation}</td>
                    <td>{stashed_order.status}</td>
                    <td>{stashed_order.bottleDescription}</td>
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
<section class="flex place-content-center h-1/2">
    <form on:submit={handleSubmit} class="grid grid-cols-2 gap-8 place-self-center">
        <label class="label" for="enteredAt">
            <span>Entered At:</span>
            <input type="datetime-local" id="enteredAt" bind:value={enteredAt} class="input" placeholder="Optional">
        </label>
    
        <label class="label" for="exitedAt">
            <span>Exited At:</span>
            <input type="datetime-local" id="exitedAt" bind:value={exitedAt} class="input" placeholder="Optional">
        </label>    
    
        <label class="label" for="controlPanelUser">
            <span>Control Panel User:</span>
            <input type="text" id="controlPanelUser" bind:value={controlPanelUser} class="input" placeholder="Optional">
        </label>
    
        <label class="label" for="oliveWashingUser">
            <span>Olive Washing User:</span> 
            <input type="text" id="oliveWashingUser" bind:value={oliveWashingUser} class="input" placeholder="Optional">
        </label>
    
        <label class="label" for="oilWaterSeparationUser">
            <span>Oil Water Separation User:</span>
            <input type="text" id="oilWaterSeparationUser" bind:value={oilWaterSeparationUser} class="input" placeholder="Optional">
        </label>
    
        <label class="label" for="order">
            <span>Order:</span>
            <input type="number" id="order" bind:value={selectedOrderId} class="input" placeholder="Select an order from the table">
        </label>
    
        <button class="btn variant-ghost col-span-2 w-72 justify-self-center" type="submit"
                >Submit</button
            >
    </form>
</section>
