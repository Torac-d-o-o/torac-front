<script lang="ts">
import { goto } from '$app/navigation'

let invalidForm = false

let username = ''
let password = ''

$: inputClasses = () => invalidForm ? 'input input-error' : 'input'

const handleSubmit = async (event: SubmitEvent) => {
  event.preventDefault()

  const response = await fetch('http://localhost:3000/user/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password })
  })

  if (response.status !== 201) {
    invalidForm = true
    return
  }

  const token = await response.text()
  document.cookie = `token=${token}`
  goto('/')
}
</script>

<section class="grid justify-center content-center h-svh">
  <form class="flex flex-col gap-4" on:submit={handleSubmit}>
    <label class="label">
      <span>Username</span>
      <input name="username" class={inputClasses()} type="text" placeholder="Your username" bind:value={username} />
    </label>

    <label class="label">
      <span>Password</span>
      <input name="password" class={inputClasses()} type="text" placeholder="Your password" bind:value={password} />
    </label>

    <button class="btn bg-gradient-to-br variant-gradient-primary-tertiary" type="submit">Login</button>

    <span class={invalidForm ? 'text-error-500 self-center' : 'text-transparent'}>
      Invalid username or password.
    </span>
  </form>
</section>
