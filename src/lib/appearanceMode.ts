import type { RequestEvent } from '@sveltejs/kit';
import { checkedWritable } from './stores/checkedWritable';

export const modeStore = checkedWritable('system', ['light', 'dark', 'system']);

export function loadMode(data: FormData, event: RequestEvent): string {
	const set = data.get('set') as string | null;
	if (set && set !== 'toggle') {
		return set;
	}

	const mode = event.cookies.get('mode');
	const hint = event.request.headers.get('Sec-CH-Prefers-Color-Scheme');
	const fallback = data.get('fallback') as string | null;
	if (mode && mode !== 'system') {
		return mode === 'dark' ? 'light' : 'dark';
	} else if (hint) {
		return hint === 'dark' ? 'light' : 'dark';
	} else if (fallback) {
		return fallback;
	} else {
		return 'dark';
	}
}
