import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import UnoCSS from 'unocss/vite';
import { extractorSvelte } from "@unocss/core";
import { presetUno, presetAttributify, presetIcons } from 'unocss';

const config: UserConfig = {
	plugins: [
		UnoCSS({
			extractors: [extractorSvelte],
			presets: [
				presetUno(),
				presetAttributify(),
				presetIcons(),
			],
		}),
		sveltekit(),
	],
};

export default config;
