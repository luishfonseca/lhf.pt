import { dev } from '$app/environment';

import { getPosts } from '$lib/db/utils';

export const load = async () => {
	return { posts: await getPosts(dev) };
};
