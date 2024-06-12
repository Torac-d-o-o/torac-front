export enum OrderStatus {
    RECEIVED = 'RECEIVED',
    STORED_DIRTY = 'STORED_DIRTY',
    IN_CLEANING = 'IN_CLEANING',
    IN_WASHING = 'IN_WASHING',
    STORED_CLEAN = 'STORED_CLEAN',
    IN_PRODUCTION = 'IN_PRODUCTION',
    IN_WEIGHING = 'IN_WEIGHING',
    IN_FILTERING = 'IN_FILTERING',
    FINISHED = 'FINISHED',
    DELIVERED = 'DELIVERED',
}

export interface Order {
    bottleDescription: string
    boxIds: number[]
    id: number
    oilFiltering: boolean
    oilWaterSeparation: boolean
    oliveAmount: number
    receivedAt: Date
    status: OrderStatus
}
