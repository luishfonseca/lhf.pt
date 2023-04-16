import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

import fs from 'fs';
import fm from 'front-matter';
import type { FrontMatterResult } from 'front-matter';
import { marked } from 'marked';

export const prerender = true;

marked.setOptions({
	walkTokens: (token) => {
		if (token.type === 'heading') {
			token.depth += 1;
		}
	}
});

const posts_dir = 'content/posts';

export const load = (({ params }) => {
	const file = `${posts_dir}/${params.path}/${params.slug}.md`;

	// Sanitize the file path
	if (
		file.includes('\0') ||
		!fs.existsSync(file) ||
		!fs.realpathSync(file).startsWith(fs.realpathSync(posts_dir))
	) {
		throw error(404, 'Not found');
	}

	const { attributes, body } = fm(fs.readFileSync(file, 'utf-8')) as FrontMatterResult<{
		title: string;
	}>;

	return {
		post: {
			title: attributes.title,
			content: marked(body)
		}
	};
}) as PageServerLoad;
