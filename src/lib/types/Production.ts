import type { Order } from './Order'

export enum ProductionStatus {
    IN_PRODUCTION = 'IN_PRODUCTION',
    OUT_PRODUCTION = 'OUT_PRODUCTION'
}

export interface Production {
    id: number
    enteredAt: Date
    exitedAt?: Date
    status: ProductionStatus
    controlPanelUser?: string,
    oliveWashingUser?: string,
    oilWaterSeparationUser?: string,
    order: Order
}