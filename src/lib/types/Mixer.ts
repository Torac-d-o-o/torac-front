import type { Production } from './Production'

export enum MixerMachines {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth
}

export enum MixerStatus {
    FULL_MIXER = 'FULL_MIXER',
    EMPTY_MIXER = 'EMPTY_MIXER'
  }

export interface Mixer {
    id: number,
    enteredAt: Date
    exitedAt?: Date
    mixerStatus: MixerStatus
    approximatedPastaWeight?: number
    machineId: MixerMachines
    productionId: number
    production: Production
}