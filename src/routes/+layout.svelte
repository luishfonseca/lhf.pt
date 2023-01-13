<script lang="ts">
	import '@unocss/reset/normalize.css';
	import 'uno.css';
	import { browser } from '$app/environment';

	let year = new Date().getFullYear();
	let darkMode = true;

	function handleToggleDarkMode() {
		darkMode = !darkMode;

		localStorage.setItem('theme', darkMode ? 'dark' : 'light');
	}

	if (browser) {
		let mediaPreference = window.matchMedia('(prefers-color-scheme: dark)').matches;
		darkMode = localStorage.theme === 'dark' || (!('theme' in localStorage) && mediaPreference);
	}
</script>

<div class={darkMode ? 'dark' : ''} text-center h-screen w-full>
	<div
		bg-white
		dark:bg-black
		text-black
		dark:text-white
		transition-colors
		transition-duration-500
		grid="~ rows-[min-content_min-content_1fr]"
		h-full
		px-8
	>
		<div pt-2 />
		<div text-right>
			<button
				on:click={handleToggleDarkMode}
				aria-label="Toggle dark mode"
				title="Toggle dark mode"
				p-6px
				text-gray4
				text-32px
				border-none
				bg-inherit
				cursor-pointer
			>
				<div dark:i-carbon-moon i-carbon-sun />
			</button>
		</div>
		<slot />
		<div font-serif text-center op-60 fw-200 py-2>
			Copyright © {year} Luís Fonseca
		</div>
	</div>
</div>

<svelte:head>
	<title>Luís Fonseca</title>
	<meta name="description" content="Luís Fonseca's personal website" />
	<meta name="author" content="Luís Fonseca" />
</svelte:head>
