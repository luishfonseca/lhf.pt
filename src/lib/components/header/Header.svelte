<script lang="ts">
	import ModeSwitcher from './ModeSwitcher.svelte';
	import SerifsSwitcher from './SerifsSwitcher.svelte';
</script>

<div id="header">
	<a href="/" class="name" id="long">Luís Fonseca</a>
	<a href="/" class="name" id="short">LHF</a>
	<div id="switchers">
		<SerifsSwitcher />
		<ModeSwitcher />
	</div>
</div>

<style lang="scss">
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

	.name {
		color: var(--text-color);
		text-decoration: none;
		font-family: var(--mono-font);
		font-size: 1.8em;
		font-weight: 200;
	}

	.name#short {
		font-size: 2em;
		// add space between letters
		letter-spacing: 0.2em;
		display: none;
	}

	@media (max-width: 768px) {
		.name#long {
			display: none;
		}

		.name#short {
			display: block;
		}
	}

	#switchers {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: fit-content;
	}
</style>
