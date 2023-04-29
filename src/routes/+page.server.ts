import { loadMode } from '$lib/stores/appearance/mode';
import { loadSerifs } from '$lib/stores/appearance/serifs';
import { redirect, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	appearance: async (event) => {
		const data = await event.request.formData();
		const location = event.request.headers.get('referer');

		let value: string;
		const key = data.get('prop');
		switch (key) {
			case 'mode':
				value = loadMode(data, event);
				break;
			case 'serifs':
				value = loadSerifs(data, event);
				break;
			default:
				return;
		}

		const expires = new Date();
		expires.setFullYear(expires.getFullYear() + 1);
		event.cookies.set(key, value, { expires, secure: false });

		throw redirect(303, location ? location : '/');
	}
};
