import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

import fs from 'fs';
import * as matter from 'gray-matter';
import { marked } from 'marked';

marked.setOptions({
    walkTokens: (token) => {
        if (token.type === 'heading') {
            token.depth += 1;
        }
    }
})

const posts_dir = 'content/posts';

export const load = (({ params }) => {

    const file = `${posts_dir}/${params.path}/${params.slug}.md`

    // Sanitize the file path
    if (file.includes('\0') || !fs.existsSync(file) || !fs.realpathSync(file).startsWith(fs.realpathSync(posts_dir))) {
        throw error(404, 'Not found')
    }

    const { data, content } = matter.read(file);

    return {
        post: {
            title: data.title,
            content: marked(content)
        }
    };
}) as PageServerLoad;