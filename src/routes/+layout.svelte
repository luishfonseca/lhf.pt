<script lang="ts">
	import type { LayoutData } from './$types';
	import { mode } from '$lib/stores/mode';

	import 'modern-normalize';
	import Header from '$lib/components/header/Header.svelte';
	import Footer from '$lib/components/footer/Footer.svelte';

	export let data: LayoutData;
	$: mode.set(data.mode);
</script>

<div id="theme-container" class={$mode}>
	<div id="app-content">
		<Header />
		<div id="main">
			<slot />
		</div>
		<Footer />
	</div>
</div>

<style lang="scss">
	@mixin light-theme() {
		--text-color: black;
		--bg-color: white;
	}

	@mixin dark-theme() {
		--text-color: white;
		--bg-color: black;
	}

	#theme-container {
		@include light-theme();
	}

	@media (prefers-color-scheme: dark) {
		#theme-container:not(.light) {
			@include dark-theme();
		}
	}

	#theme-container.dark {
		@include dark-theme();
	}

	#app-content {
		background-color: var(--bg-color);
		color: var(--text-color);
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100vh;
	}

	#main {
		height: 100%;
		width: clamp(200px, 50%, 800px);
		display: flex;
		flex-direction: row;
	}
</style>
