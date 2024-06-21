<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import type { PageData } from './$types'
    import data from '../order/+page.svelte'
    import { onMount } from "svelte"

    let token = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.j6kw2giRmlWJezLhjYMp9zugaSFlBdLLfJ7Fomw-ceEGQGbRrySD2BeHSSdmJ21HeiII1Qzq6QfJZBdyA667AQ"

</script>


<div class="flex justify-center">Hello</div>