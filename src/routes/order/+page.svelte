<script lang="ts">
    import { invoke } from '@tauri-apps/api'
    import DownArrow from '$lib/components/icons/downArrow.svelte'

    import type { PageData } from './$types'
    import type { Customer, Order } from '$lib/types'
    import { getCookieAttribute } from '$lib/utils/cookie_parser'

    //export let data: PageData

    const status = 'RECEIVED'
    const workerUsername = window.localStorage.getItem('username')

    let bottlesDescription: string
    let boxIds: string
    let customerId: string | number
    let oilFiltering: string
    let oilWaterSeparation: string
    let oliveAmount: number

    let customers: Customer[] | null = null

    const token = window.localStorage.getItem("token")
    
    invoke('get_customers', { token: token }).then((data) => {
        customers = JSON.parse(data as string)
        if (customers) customerId = customers[0].id
    })

    let formError: string | null = null

    $: inputClasses = formError ? 'input input-error' : 'input'

    const handleSubmit = async (event: SubmitEvent) => {
        // Stop the form event.
        event.preventDefault()

        const response: string = await invoke('order_register', {
            data: {
                bottlesDescription,
                boxIds: boxIds.split(/\s*,\s*/).map((id) => Number.parseInt(id)),
                customerId: Number.parseInt(customerId as string),
                oilFiltering: oilFiltering === '1',
                oilWaterSeparation: oilWaterSeparation === '1',
                oliveAmount,
                receivedAt: Date.now(),
                status,
                workerUsername
            },
            token: token
        })
        console.log(JSON.parse(response))
        if (!response) {
            formError = 'Failed to register order.'
            return
        }

        bottlesDescription = ''
        boxIds = ''
        oliveAmount = 0
    }

    $: drawerOpen = false
    let orderList: Order[] = []

    const openDrawer = async () => {
        drawerOpen = !drawerOpen
        orderList = JSON.parse(
            await invoke('get_orders', {
                token: token,
                username: window.localStorage.getItem('username')
            })
        )
    }
</script>

<section class="flex place-content-center h-dvh">
    <form class="grid grid-cols-2 gap-8 place-self-center" on:submit={handleSubmit}>
        <label class="label">
            <span>Bottle description</span>
            <input
                name="bottlesDescription"
                class={inputClasses}
                type="text"
                placeholder="Optional description"
                bind:value={bottlesDescription}
            />
        </label>

        <label class="label">
            <span>Box ID</span>
            <input
                name="boxID"
                class={inputClasses}
                type="text"
                placeholder="372819, 8492189, 1943782"
                bind:value={boxIds}
            />
        </label>

        <label class="label">
            <span>Customer</span>
            <select class="select select-error" bind:value={customerId}>
                {#if customers === null}
                    <option value="0">Loading...</option>
                {:else}
                    {#each customers as customer}
                        <option value={customer.id}>{customer.nameSurname}</option>
                    {/each}
                {/if}
            </select>
        </label>

        <label class="label">
            <span>Oil filtering</span>
            <select name="oilFiltering" class="select" bind:value={oilFiltering}>
                <option value="0">No</option>
                <option value="1">Yes</option>
            </select>
        </label>

        <label class="label">
            <span>Oil and water separation</span>
            <select name="oilWaterSeparation" class="select" bind:value={oilWaterSeparation}>
                <option value="0">No</option>
                <option value="1">Yes</option>
            </select>
        </label>

        <label class="label">
            <span>Amount of olives</span>
            <input name="oliveAmount" class={inputClasses} type="number" bind:value={oliveAmount} />
        </label>

        <button class="btn variant-ghost col-span-2 w-72 justify-self-center" type="submit"
            >Submit</button
        >
    </form>

    <button
        class="fixed bottom-0 mb-4 p-2 rounded-full border-2 border-black dark:border-white"
        on:click={openDrawer}
    >
        <DownArrow />
    </button>
    <div class="fixed bottom-0">
        {#each orderList as order}
            <span>{order.id}</span>
        {/each}
    </div>
</section>
