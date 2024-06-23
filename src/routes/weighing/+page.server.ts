import { redirect } from '@sveltejs/kit'

import type { PageServerLoad } from './$types'

export const load: PageServerLoad = ({ cookies }) => {
    const token = cookies.get('token')

    if (!token) redirect(307, '/login')

    return { token }
}
