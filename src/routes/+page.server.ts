import { loadMode } from '$lib/appearanceMode';
import { loadSerifs } from '$lib/appearanceSerifs';
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

		let expires = new Date();
		expires.setFullYear(expires.getFullYear() + 1);
		event.cookies.set(key, value, { expires, secure: false });

		throw redirect(303, location ? location : '/');
	}
};
