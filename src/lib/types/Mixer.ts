export enum MixerMachines {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth
}

export interface Mixer {
    id: number,
    enteredAt: Date
    exitedAt?: Date
    approximatedPastaWeight?: number
    machineId: MixerMachines
    productionId: number
}