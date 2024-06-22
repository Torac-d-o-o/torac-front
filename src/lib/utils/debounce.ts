export function debounce(cb: CallableFunction, delay: number) {
    let timeout: string | number | NodeJS.Timeout | undefined
    return (...args: Array<string>) => {
        clearTimeout(timeout)
        timeout = setTimeout(() => {
            cb(...args)
        }, delay)
    }
}