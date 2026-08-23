import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ url }) => {
  // Check for access token in localStorage (client-side guard)
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('kagua_access_token');
    if (!token) {
      throw redirect(302, `/login?redirectTo=${encodeURIComponent(url.pathname)}`);
    }
  }
  return {};
};
