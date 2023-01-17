<script lang="ts">
	import ModeSwitcher from './ModeSwitcher.svelte';
	import SerifsSwitcher from './SerifsSwitcher.svelte';
</script>

<div id="header">
	<a href="/" id="name">Luís Fonseca</a>
	<div id="switchers">
		<SerifsSwitcher />
		<ModeSwitcher />
	</div>
</div>

<style>
	#header {
		transition: background-color 0.2s ease-in-out;
		background-color: var(--base-color);
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		height: fit-content;
		padding: 10px 16px;
	}

	#name {
		color: var(--text-color);
		text-decoration: none;
		font-family: var(--mono-font);
		font-size: 1.8em;
		font-weight: 200;
	}
	#switchers {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: fit-content;
	}
</style>
