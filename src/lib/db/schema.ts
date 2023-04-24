import {
	mysqlTable,
	varchar,
	text,
	boolean,
	timestamp,
	primaryKey,
	index
} from 'drizzle-orm/mysql-core';
import { sql } from 'drizzle-orm';

export const post = mysqlTable(
	'Post',
	{
		slug: varchar('slug', { length: 256 }).primaryKey(),
		title: varchar('title', { length: 256 }).notNull(),
		content: text('content'),
		published: boolean('published').notNull().default(false),
		createdAt: timestamp('created_at', { fsp: 2 })
			.notNull()
			.default(sql`now(2)`)
	},
	(table) => ({
		createdAtIdx: index('created_at_idx').on(table.createdAt)
	})
);

export const tag = mysqlTable('Tag', {
	name: varchar('name', { length: 256 }).primaryKey()
});

export const tagsOnPosts = mysqlTable(
	'TagsOnPosts',
	{
		postSlug: varchar('post_slug', { length: 256 }).notNull(),
		tagName: varchar('tag_name', { length: 256 }).notNull()
	},
	(table) => ({
		tagsOnPostsKey: primaryKey(table.postSlug, table.tagName)
	})
);
