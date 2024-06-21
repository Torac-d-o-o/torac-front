export function getCookieAttribute(attributeName: string) {
    const match: RegExpMatchArray | null = document.cookie.match(new RegExp('(^| )' + attributeName + '=([^;]+)'))
    if (match) {
        return match[2]
    }
    return null
}
