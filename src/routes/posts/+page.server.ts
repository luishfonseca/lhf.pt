import fs from 'fs';

export const prerender = true;

const posts_dir = 'content/posts';

function getFiles(dir: fs.PathLike): string[] {
	return fs.readdirSync(dir).flatMap((file) => {
		const path = `${dir}/${file}`;
		return fs.statSync(path).isDirectory() ? getFiles(path) : path;
	});
}

export const load = () => {
	return {
		posts: getFiles(posts_dir).map((file) => {
			const dirs = file.split('/');
			return [
				...dirs.slice(2, dirs.length - 1),
				'-',
				dirs[dirs.length - 1].replace('.md', '')
			].join('/');
		})
	};
};
