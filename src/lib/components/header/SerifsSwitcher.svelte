<script lang="ts">
	import { enhance } from '$app/forms';
	import { serifsStore } from '$lib/appearanceSerifs';
	import './switcher.css';
</script>

<form method="POST" action="/?/appearance" use:enhance>
	<input type="hidden" name="prop" value="serifs" />
	<input type="hidden" name="set" value="toggle" />
	<button title="Toggle Serifs" aria-label="Toggle Serifs" formaction="/?/appearance">
		<div style="font-family: var({$serifsStore === 'yes' ? '--sans-font' : '--serif-font'})">F</div>
	</button>
</form>

<style lang="scss">
	div {
		font-size: 32px;
		font-weight: 100;
		color: var(--text-color);
	}
</style>
