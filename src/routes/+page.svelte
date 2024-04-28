<script lang='ts'>
  import { goto } from '$app/navigation'
    import '../app.css'

    let username: string = "";
    let password: string = "";

    function logIn() {
        goto("/home");
    }

</script>

<div class="flex justify-center items-center content-center h-screen">
    <div class="flex flex-col items-center bg-primary w-2/5 rounded-xl h-2/3">
        <h1 class="relative border-2 border-black top-10 text-5xl">Log in</h1>
        <form action="" on:submit|preventDefault={logIn} class="relative border-2 w-full border-black flex flex-col space-y-10 h-full items-center justify-center">
            <div class="border-2 border-black w-full h-1/5 flex flex-row justify-center items-center">
                <label for="" class="text-2xl mr-5">Username: </label>
                <input type="text" class="h-2/5 text-2xl pl-4 w-2/5" value={username}>
            </div>
            <div class="border-2 border-black w-full h-1/5 flex flex-row justify-center items-center">
                <label for="" class="text-2xl mr-5">Password: </label>
                <input type="password" class="h-2/5 text-2xl pl-5 w-2/5" value={password}>
            </div>
            <button type="submit" class="h-1/5 border-2 w-2/5 border-black text-4xl bg-secondary hover:cursor-pointer rounded-xl">Log in</button>
        </form>
    </div>
    
</div>



