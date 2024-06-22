<script lang="ts">
    import { event, invoke } from "@tauri-apps/api"
    import { type Customer, type Order } from "$lib/types"
    import { onMount } from "svelte"
    import { getCookieAttribute } from "$lib/utils/cookie_parser"
    import { formatDateTime } from "$lib/utils/date_formater"
    import { debounce } from "$lib/utils/debounce"

    let nameSurname: string = ''
    let address: string = ''
    let postalCode: number | null = null
    let phoneNumber: string = '' 
    let searchNameInput: string = ''
    let searchAddressInput: string = ''
    let foundCustomers: Customer[] | null = null
    let orderData: Order[] | null = null
    let selectedName: string = ''
    let selectedAddress: string = ''

    const token = getCookieAttribute('token');

    onMount(() => {

    })

    // Debounced search function
    const searchCustomersName = debounce(async () => {
        try {
            const data = await invoke('get_customers', { token, customerName: searchNameInput });
            if (data !== null) {
                foundCustomers = JSON.parse(data as string)
                console.log(foundCustomers)
            }
        } catch (error) {
            console.error('Error fetching customers:', error)
        }
    }, 500) // Adjust delay as needed

    function handleNameInput (event: any) {
        if (event.target !== null) {
            searchNameInput = event.target.value
            searchCustomersName()
        }
    }

    async function getCustomersOrders() {
        invoke('get_orders', {token: token, customerName: searchNameInput, customerAddress: searchAddressInput})
        .then((data) => { 
            if (data !== null) {
                orderData = JSON.parse(data as string)
                console.log(orderData)
            }            
        })
    }

    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault()

        const response: string = await invoke('customer_register', {
            data: {
                nameSurname: nameSurname,
                address: address,
                postalCode: postalCode,
                phoneNumber: phoneNumber,
            },
            token: token
        })
        if (response) {
            nameSurname = ''
            address = ''
            postalCode = null
            phoneNumber = ''
        }
    }
</script>


<div class="grid grid-rows-2 gap-4 place-content-center">
    <form class="flex flex-col gap-4" on:submit={handleSubmit}>
        <label>
        Name Surname:
        <input class="input" type="text" bind:value={nameSurname} required />
        </label>
        
        <label>
        Address:
        <input class="input" type="text" bind:value={address} required />
        </label>
        
        <label>
        Postal Code:
        <input class="input" type="number" bind:value={postalCode} required />
        </label>
        
        <label>
        Phone Number:
        <input class="input" type="text" bind:value={phoneNumber} required />
        </label>
        
        <button class="btn variant-ghost" type="submit">Add Customer</button>
    </form>
    <div class="flex flex-col gap-4">
        <form class="flex flex-row gap-4" on:submit={getCustomersOrders}>
            <div class="grid grid-cols-1">
                <label for="">Search name:
                    <input class="input" type="text" bind:value={searchNameInput} on:input={handleNameInput}>
                </label>
                {#if foundCustomers !== null}
                    <select class="select select-error" bind:value={selectedName} on:change={() => {searchNameInput = selectedName}}>
                        {#each foundCustomers as customer}
                            <option value="{customer.nameSurname}">{customer.nameSurname}</option>
                        {/each}
                    </select>
                {/if}
                <label for="">Search address:
                    <input class="input" type="text" bind:value={searchAddressInput}>
                </label>
                {#if foundCustomers !== null}
                    <select class="select select-error" bind:value={selectedAddress} on:change={() => {searchAddressInput = selectedAddress}}>
                        {#each foundCustomers as customer}
                            <option value="{customer.address}">{customer.address}</option>
                        {/each}
                    </select>
                {/if}
            </div>
            <button class="btn variant-ghost" type="submit">Search</button>
        </form>
        <div class="table-container">
            <table class="table table-hover table-interactive">
                <thead class="text-l my-2">
                    <tr class="flex justify-center">
                        Customer data
                    </tr>
                    <tr>
                        <td>Name Surname</td>
                        <td>Order ID</td>
                        <td>Date</td>
                        <td>Olives brought</td>
                        <td>Order status</td>
                        <td>Oil made</td>
                        <td>Ratio</td>
                    </tr>
                </thead>
                <tbody>
                    {#if orderData !== null}
                        {#each orderData as order}
                            <tr>
                                <td>{selectedName}</td>
                                <td>{order.id}</td>
                                <td>{formatDateTime(order.receivedAt.toString())}</td>
                                <td>{order.oliveAmount}</td>
                                <td>{order.status}</td>
                                <td></td>
                                <td></td>
                            </tr>
                        {/each}    
                    {/if}
                </tbody>
            </table>
        </div>
    </div>
</div>
