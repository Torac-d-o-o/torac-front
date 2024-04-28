import { writable } from "svelte/store";

export let loggedInStore = writable({
    loggedIn: true,
    tag: ""
});