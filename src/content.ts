import { drizzle, type MySql2Database } from 'drizzle-orm/mysql2';
import mysql from 'mysql2/promise';
import * as fs from 'fs';
import matter from 'gray-matter';
import { marked } from 'marked';

import { eq } from 'drizzle-orm';
import { post } from './lib/db/schema';

const POSTS_DIR = './content/posts/';

if (!process.env.DATABASE_URL) {
	console.log('Missing DATABASE_URL environment variable');
	process.exit(1);
}

const connection = await mysql.createConnection(process.env.DATABASE_URL);

const db = drizzle(connection, { logger: true });

interface Commands {
	[key: string]: (confirm: boolean) => Promise<void>;
}

const parsePost = (slug: string) => {
	const raw = fs.readFileSync(POSTS_DIR + slug + '.md');
	const fm = matter(raw);
	const content = marked.parse(fm.content);
	return { title: fm.data.title, published: fm.data.published, content };
};

const commands: Commands = {
	publish: async (_) => {
		const posts = await db.select().from(post);

		fs.readdirSync(POSTS_DIR).forEach(async (postPath) => {
			const lastModified = fs.statSync(POSTS_DIR + postPath).mtime;
			const slug = postPath.replace('.md', '');
			const data = parsePost(slug);
			const p = posts.find((post) => post.slug === slug);

			if (p && p.updatedAt < lastModified) {
				await db
					.update(post)
					.set({
						content: data.content,
						title: data.title,
						published: data.published,
						updatedAt: new Date()
					})
					.where(eq(post.slug, slug));
			} else if (!p) {
				await db.insert(post).values({
					slug: slug,
					content: data.content,
					title: data.title,
					published: data.published,
					special: false
				});
			}
		});
	},
	prune: async (confirm) => {
		if (confirm) {
			let op;
			try {
				op = fs.readFileSync('.content-op', 'utf-8').split(';');
			} catch (err) {
				console.error('No operation waiting for confirmation');
				return;
			}

			if (op[0] !== 'prune') {
				console.error('No prune operation waiting for confirmation');
				return;
			}

			// FIXME: this is only deleting one at a time???
			op.slice(1).forEach(async (slug) => {
				await db.delete(post).where(eq(post.slug, slug));
			});

			fs.rmSync('.content-op');
			return;
		}

		const posts = await db.select().from(post).where(eq(post.special, false));

		let missing: string[] = [];
		posts.forEach(async (p) => {
			try {
				fs.accessSync(POSTS_DIR + p.slug + '.md');
			} catch (err: any) {
				if (err.code === 'ENOENT') {
					missing.push(p.slug);
				}
			}
		});

		if (missing.length !== 0) {
			fs.writeFileSync('.content-op', 'prune;' + missing.join(';'));
			console.log('\nThe following posts will be removed: ' + missing.join(', '));
			console.log('Run with CONFIRM=y to confirm this operation');
		} else {
			console.log('\nNothing to prune');
		}
	}
};

const argv = process.argv.slice(2);
if (argv.length === 0) {
	console.log(
		'Usage: ts-node src/content.ts <command>\nCommands: ' + Object.keys(commands).join(' | ')
	);
	process.exit(1);
}

const cmd = argv[0];
if (!commands.hasOwnProperty(cmd)) {
	console.log(`Unknown command: ${cmd}\nCommands: ` + Object.keys(commands).join(' | '));
	process.exit(1);
}

commands[cmd](process.env.hasOwnProperty('CONFIRM')).finally(() => {
	process.exit();
});
