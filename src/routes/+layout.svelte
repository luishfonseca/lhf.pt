<script lang="ts">
	import type { LayoutData } from './$types';
	import { modeStore } from '$lib/appearanceMode';

	import '@fontsource/ibm-plex-sans';
	import '@fontsource/ibm-plex-serif';
	import 'modern-normalize';
	import Header from '$lib/components/header/Header.svelte';
	import { serifsStore } from '$lib/appearanceSerifs';

	export let data: LayoutData;
	$: modeStore.set(data.mode);
	$: serifsStore.set(data.serifs);
</script>

<div id="appearance-container" class="{$modeStore} {$serifsStore === 'yes' ? 'serifs' : ''}">
	<div id="app-content">
		<Header />
		<slot />
	</div>
</div>

<style lang="scss">
	@mixin rose-pine() {
		--base-color: #faf4ed;
		--bg-color: #fffaf3;
		--overlay-color: #f2e9e1;
		--muted-color: #9893a5;
		--subtle-color: #797593;
		--text-color: #575279;
		--red: #b4637a;
		--yellow: #ea9d34;
		--blue: #286983;
		--lblue: #56949f;
		--purple: #907aa9;
		--base-color-dark: #191724;
		--bg-color-dark: #1f1d2e;
		--overlay-color-dark: #26233a;
		--muted-color-dark: #6e6a86;
		--subtle-color-dark: #908caa;
		--text-color-dark: #e0def4;
		--red-dark: #eb6f92;
		--yellow-dark: #f6c177;
		--blue-dark: #31748f;
		--lblue-dark: #9ccfd8;
		--purple-dark: #c4a7e7;
	}

	@mixin dark-mode() {
		--base-color: var(--base-color-dark);
		--bg-color: var(--bg-color-dark);
		--overlay-color: var(--overlay-color-dark);
		--muted-color: var(--muted-color-dark);
		--subtle-color: var(--subtle-color-dark);
		--text-color: var(--text-color-dark);
		--red: var(--red-dark);
		--yellow: var(--yellow-dark);
		--blue: var(--blue-dark);
		--lblue: var(--lblue-dark);
		--purple: var(--purple-dark);
	}

	#appearance-container {
		--sans-font: 'IBM Plex Sans', sans-serif;
		--serif-font: 'IBM Plex Serif', serif;
		--mono-font: 'IBM Plex Mono', monospace;
		font-family: var(--sans-font);

		@include rose-pine();
	}

	@media (prefers-color-scheme: dark) {
		#appearance-container:not(.light) {
			@include dark-mode();
		}
	}

	#appearance-container.dark {
		@include dark-mode();
	}

	#appearance-container.serifs {
		font-family: var(--serif-font);
	}

	#app-content {
		transition: background-color 0.2s ease-in-out;
		background-color: var(--bg-color);
		color: var(--text-color);
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100vh;
	}
</style>
