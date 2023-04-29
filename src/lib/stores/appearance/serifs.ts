import type { RequestEvent } from '@sveltejs/kit';
import { checkedWritable } from '../checkedWritable';

export const serifsStore = checkedWritable('', ['serifs', '']);

export function loadSerifs(data: FormData, event: RequestEvent): string {
	const set = data.get('set') as string | null;
	if (set && set !== 'toggle') {
		return set;
	}

	const serifs = event.cookies.get('serifs');
	if (serifs) {
		return serifs === '' ? 'serifs' : '';
	} else {
		return 'serifs';
	}
}
