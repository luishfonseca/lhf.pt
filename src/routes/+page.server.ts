import { loadMode } from "$lib/appearanceMode";
import { redirect, type Actions, type ServerLoadEvent } from "@sveltejs/kit";

export async function load(event: ServerLoadEvent) {
    event.setHeaders({
        // TODO: This header is https only, make sure that the nginx proxy doesn't remove it
        'Accept-CH': 'Sec-CH-Prefers-Color-Scheme'
    });
}

export const actions: Actions = {
    appearance: async event => {
        const data = await event.request.formData();
        const location = event.request.headers.get('origin');

        let value: string;
        const key = data.get('prop');
        switch (key) {
            case 'mode':
                value = loadMode(data, event);
                break;
            default:
                return;
        }

        let expires = new Date();
        expires.setFullYear(expires.getFullYear() + 1);
        event.cookies.set(key, value, { expires });

        throw redirect(303, location ? location : '/')
    },
};