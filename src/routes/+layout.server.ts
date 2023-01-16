import type { LayoutServerLoadEvent } from './$types';

export const load = async (event: LayoutServerLoadEvent) => {
	return {
		mode: event.cookies.get('mode')
	};
};
