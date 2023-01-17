<script lang="ts">
	import Icon, { type IconifyIcon } from '@iconify/svelte/dist/OfflineIcon.svelte';
	import { enhance } from '$app/forms';
	import { browser } from '$app/environment';
	import { modeStore } from '$lib/appearanceMode';

	import './switcher.css';
	import sun from '@iconify-icons/carbon/sun';
	import moon from '@iconify-icons/carbon/moon';

	// Tells server what to set as the mode if 'toggle' fails
	// This is useful when there is no cookie and Content Hints are not supported.
	function getFallback() {
		const dark = browser && window.matchMedia('(prefers-color-scheme: dark)').matches;
		return dark ? 'light' : 'dark';
	}

	function getIcon(mode: string): IconifyIcon {
		mode = mode !== 'system' ? mode : getFallback();
		return mode === 'dark' ? sun : moon;
	}
</script>

<form method="POST" action="/?/appearance" use:enhance>
	<input type="hidden" name="prop" value="mode" />
	<input type="hidden" name="set" value="toggle" />
	<input type="hidden" name="fallback" value={getFallback()} />
	<button title="Toggle Dark mode" aria-label="Toggle Dark mode" formaction="/?/appearance">
		<Icon icon={getIcon($modeStore)} height="32px" color="var(--text-color)" />
	</button>
</form>
