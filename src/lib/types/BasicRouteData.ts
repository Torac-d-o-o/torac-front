import type { ComponentType } from 'svelte'

export interface BasicRouteData {
    name: string
    route: string
    icon?: ComponentType
}
