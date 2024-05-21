import { redirect } from '@sveltejs/kit'

import type { RequestEvent } from '@sveltejs/kit'

export function load({ cookies }: RequestEvent) {
  const token = cookies.get('token')

  if (!token) redirect(307, '/login')
}
