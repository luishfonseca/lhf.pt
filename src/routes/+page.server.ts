import { redirect, type Actions, type ServerLoadEvent } from "@sveltejs/kit";

export async function load(event: ServerLoadEvent) {
    event.setHeaders({
        'Accept-CH': 'Sec-CH-Prefers-Color-Scheme'
    });
}

export const actions: Actions = {
    appearance: async ({ cookies, request }) => {
        const data = await request.formData();
        const location = request.headers.get('origin');

        let value: string;
        const key = data.get('prop');
        switch (key) {
            case 'mode':
                const set = data.get('set') as string | null;
                if (set && set !== 'toggle') {
                    value = set;
                    break;
                }

                const mode = cookies.get('mode');
                const hint = request.headers.get('Sec-CH-Prefers-Color-Scheme');
                const fallback = data.get('fallback') as string | null;
                if (mode && mode !== 'system') {
                    value = mode === 'dark' ? 'light' : 'dark';
                } else if (hint) {
                    value = hint === 'dark' ? 'light' : 'dark';
                } else if (fallback) {
                    value = fallback;
                } else {
                    value = 'dark';
                }
                break;
            default:
                return;
        }

        let expires = new Date();
        expires.setFullYear(expires.getFullYear() + 1);
        cookies.set(key, value, { expires });

        throw redirect(303, location ? location : '/')
    },
};