// src/lib/utils/date.ts
import { format } from 'date-fns'

export function formatDateTime(dateString: string): string {
    return format(new Date(dateString), 'MMMM d, yyyy, h:mm a')
}