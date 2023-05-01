import { drizzle, type MySql2Database } from 'drizzle-orm/mysql2';
import mysql from 'mysql2/promise';
import * as fs from 'fs';
import matter from 'gray-matter';
import { marked } from 'marked';

import { eq } from 'drizzle-orm';
import { post } from './lib/db/schema';

const POSTS_DIR = './content/posts/';

interface Commands {
    [key: string]: (db: MySql2Database) => Promise<void>;
}

const parsePost = (slug: string) => {
    const raw = fs.readFileSync(POSTS_DIR + slug + '.md');
    const fm = matter(raw);
    const content = marked.parse(fm.content);
    return { title: fm.data.title, published: fm.data.published, content };
};

const commands: Commands = {
    publish: async (db) => {
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
                await db
                    .insert(post)
                    .values({
                        slug: slug,
                        content: data.content,
                        title: data.title,
                        published: data.published,
                        special: false
                    })
            }
        });
    }
};

const argv = process.argv.slice(2);
if (argv.length !== 1) {
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

if (!process.env.DATABASE_URL) {
    console.log('Missing DATABASE_URL environment variable');
    process.exit(1);
}

const connection = await mysql.createConnection(process.env.DATABASE_URL);

const db = drizzle(connection, { logger: true });

commands[cmd](db).finally(() => {
    process.exit();
});
