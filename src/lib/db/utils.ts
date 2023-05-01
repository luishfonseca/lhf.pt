import { connect } from '@planetscale/database';
import { drizzle } from 'drizzle-orm/planetscale-serverless';

import { DATABASE_HOST, DATABASE_USERNAME, DATABASE_PASSWORD } from '$env/static/private';

import { eq, and, desc } from 'drizzle-orm';
import { post } from '$lib/db/schema';

const connection = connect({
	host: DATABASE_HOST,
	username: DATABASE_USERNAME,
	password: DATABASE_PASSWORD
});

const db = drizzle(connection);

export const getPost = async (slug: string) => {
	return (
		await db
			.select()
			.from(post)
			.where(and(eq(post.special, false), eq(post.slug, slug)))
			.limit(1)
	)[0];
};

export const getPosts = async (includeDrafts: boolean) => {
	return await db
		.select()
		.from(post)
		.where(and(eq(post.special, false), eq(post.published, includeDrafts ? false : true)))
		.orderBy(desc(post.createdAt));
};
