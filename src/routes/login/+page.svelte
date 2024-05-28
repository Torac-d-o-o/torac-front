<script lang="ts">
    import { goto } from '$app/navigation'
    import { invoke } from '@tauri-apps/api'

    /** The error to be shown to user based on what request they do. */
    let formError: string | null = null

    let username = ''
    let password = ''

    /** Disables all the inputs so the user can't spam requests. */
    let disableInputs = false

    /** The base classes to give inputs. */
    $: inputClasses = () => (formError ? 'input input-error' : 'input')

    const handleSubmit = async (event: SubmitEvent) => {
        // Stop the form event.
        event.preventDefault()

        // Disable inputs.
        disableInputs = true

        const buttonId = event.submitter!.id

        const response: string = await invoke(buttonId, { username, password })
        disableInputs = false

        if (!response) {
            formError = buttonId === 'login'
                ? 'Invalid username or password.'
                : `User '${username}' already exists.`
            return
        }

        // We set a cookie manually as the back-end does not have a login page to make a proper HTTP-Only cookie.
        document.cookie = `token=${response};samesite=lax`
        goto('/')
    }
</script>

<section class="grid justify-center content-center h-svh">
    <form class="flex flex-col gap-4" on:submit={handleSubmit}>
        <label class="label">
            <span>Username</span>
            <input
                name="username"
                class={inputClasses() + ' max-w-72'}
                type="text"
                placeholder="Your username"
                bind:value={username}
                disabled={disableInputs}
            />
        </label>

        <label class="label">
            <span>Password</span>
            <input
                name="password"
                class={inputClasses() + ' max-w-72'}
                type="text"
                placeholder="Your password"
                bind:value={password}
                disabled={disableInputs}
            />
        </label>

        <div class="flex gap-8 place-self-center">
            <button
                id="login"
                class="btn bg-gradient-to-br variant-gradient-primary-tertiary w-32"
                type="submit"
                disabled={disableInputs}
            >
                Login
            </button>
            <button
                id="register"
                class="btn variant-ghost w-32"
                disabled={disableInputs}
            >
                Register
            </button>
        </div>

        <span class={formError ? 'text-error-500 self-center w-fit' : 'text-transparent'}>
            {formError}
        </span>
    </form>
</section>
