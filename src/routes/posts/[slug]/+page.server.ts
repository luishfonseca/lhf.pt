import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { dev } from '$app/environment';
import { getPost } from '$lib/server/database';

export const load = (async ({ params }) => {
	const post = await getPost(params.slug);

	if (!post || (!dev && !post.published)) {
		throw error(404, 'Not found');
	}

	return { post };
}) as PageServerLoad;
