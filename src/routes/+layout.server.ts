import type { LayoutServerLoadEvent } from './$types';

export const load = async (event: LayoutServerLoadEvent) => {
	event.setHeaders({
		// TODO: This header is https only, make sure that the nginx proxy doesn't remove it
		'Accept-CH': 'Sec-CH-Prefers-Color-Scheme'
	});

	return {
		mode: event.cookies.get('mode'),
		serifs: event.cookies.get('serifs')
	};
};
