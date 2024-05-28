<script lang="ts">
  import { goto } from "$app/navigation"
  import { writable } from "svelte/store"

  let username = '';
  let password = '';
  const errorMessage = writable('');

async function logIn(event: Event): Promise<void> {
    event.preventDefault()
    errorMessage.set('')
    const response = await fetch('http://localhost:3000/user/login', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({ username, password })
    })
    .then((data) => {
    if (data) {
        setTimeout(() => goto('/reception'), 0)
    }
    })
}

</script>

<div class="flex justify-center items-center h-screen">
    <div class="p-12 rounded-lg shadow-lg w-1/2">
        <h2 class="text-4xl mb-14 flex justify-center">Log in</h2>
        <form on:submit={logIn}>
            <div class="mb-8">
                <label for="username" class="label block text-2xl mb-1 font-medium">Username</label>
                <input type="text" id="username" class="input w-full px-6 py-4 border rounded-lg text-xl" placeholder="Enter your username" bind:value={username}>
            </div>
            <div class="mb-8">
                <label for="password" class="label block text-2xl mb-1 font-medium">Password</label>
                <input type="password" id="password" class="input w-full px-6 py-4 border rounded-lg text-xl" placeholder="Enter your password" bind:value={password}>
            </div>
            <button type="submit" class="w-full py-4  bg-tertiary-500 rounded-lg text-2xl">Log in</button>
        </form>
    </div>
</div>


