import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import UnoCSS from 'unocss/vite';
import { extractorSvelte } from "@unocss/core";
import { presetUno, presetAttributify, presetIcons, presetWebFonts } from 'unocss';

const config: UserConfig = {
	plugins: [
		UnoCSS({
			extractors: [extractorSvelte],
			presets: [
				presetUno(),
				presetWebFonts({
					provider: 'bunny',
					fonts: {
						sans: 'IBM Plex Sans',
						mono: 'IBM Plex Mono',
						serif: 'IBM Plex Serif',
					}
				}),
				presetAttributify(),
				presetIcons(),
			],
		}),
		sveltekit(),
	],
};

export default config;
