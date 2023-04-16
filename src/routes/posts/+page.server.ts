import { dev } from '$app/environment';

import { getPosts } from '$lib/server/database';

export const load = async () => {
	return { posts: await getPosts(dev) };
};
