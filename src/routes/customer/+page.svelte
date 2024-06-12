<script lang="ts">
    import { invoke } from "@tauri-apps/api"
    import { OrderStatus, type Order } from "$lib/types"
    import type { PageData } from './$types'
    import data from '../order/+page.svelte'
    import { onMount } from "svelte"

    let token = "eyJhbGciOiJFZERTQSJ9.eyJ1c2VybmFtZSI6IlZpbmtvIiwicGFzc3dvcmQiOiJMaWdtYSIsImZsYWdzIjo4fQ.pu_0ulHSFflnkiwT09tHq2nIsViM8XYIqBsxe1hs-qKZ-qU-mVbjBdfNsgkNvQ-WxusKXGhy79MWdMRTVi2sCA"

</script>


<div class="flex justify-center">Hello</div>