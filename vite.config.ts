import { sveltekit } from '@sveltejs/kit/vite';
import { loadEnv, defineConfig } from 'vite';

export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, process.cwd(), '');
	return {
		plugins: [sveltekit()],
		define: {
			'import.meta.env.VERCEL_ANALYTICS_ID': JSON.stringify(env.VERCEL_ANALYTICS_ID)
		}
	};
});
