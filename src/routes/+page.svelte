<script>
	import Icon from '@iconify/svelte/dist/OfflineIcon.svelte';
	import dotMark from '@iconify-icons/carbon/dot-mark';
	import github from '@iconify-icons/carbon/logo-github';
	import linkedin from '@iconify-icons/carbon/logo-linkedin';
	import email from '@iconify-icons/carbon/email';

	let links = [
		{
			name: 'About',
			href: '/about'
		},
		{
			name: 'CV',
			href: '/cv'
		},
		{
			name: 'Posts',
			href: '/posts'
		}
	];

	let social = [
		{
			icon: github,
			href: 'https://github.com/luishfonseca'
		},
		{
			icon: email,
			href: 'mailto:luis@lhf.pt',
			target: '_blank'
		},
		{
			icon: linkedin,
			href: 'https://www.linkedin.com/in/luishfonseca/'
		}
	];
</script>

<svelte:head>
	<title>Luís Fonseca's Blog</title>
	<meta name="description" content="Luís Fonseca's personal website" />
	<meta name="author" content="Luís Fonseca" />
</svelte:head>

<div id="container">
	<div id="links">
		{#each links as { href, name }, i}
			<div>
				<a {href}>{name}</a>
			</div>
			{#if i < links.length - 1}
				<div class="separator"><Icon icon={dotMark} /></div>
			{/if}
		{/each}
	</div>
	<div id="social">
		{#each social as { href, icon, target }}
			<a {href} {target}>
				<Icon height="64px" {icon} />
			</a>
		{/each}
	</div>
</div>

<style lang="scss">
	$accent-color: var(--purple);

	#container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		width: 100%;
		height: 100%;
		gap: 50px;
	}

	.separator {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	#links {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 0 12px;
		color: var(--muted-color);

		> :first-child {
			text-align: right;
		}

		> :first-child,
		> :last-child {
			flex-grow: 1;
			flex-basis: 0;
		}

		> div {
			> a {
				font-size: 3em;
				font-weight: 400;
				color: var(--subtle-color);
				padding: 0 4px;
				border-bottom: dotted 5px transparent;
				transition: all 0.2s ease-in-out;
				text-decoration: none;
			}

			> a:hover,
			> a:focus {
				color: $accent-color;
				border-bottom: dotted 5px;
				transition: color 0.2s ease-in-out;
			}
		}
	}

	#social {
		display: flex;
		justify-content: center;
		gap: 20px;

		> a {
			color: var(--subtle-color);

			&:hover,
			&:focus {
				color: $accent-color;
				transition: color 0.2s ease-in-out;
			}
		}
	}

	@media (max-width: 768px) {
		#links > * {
			text-align: center !important;
		}

		#links {
			flex-direction: column;
		}

		.separator {
			visibility: hidden;
		}
	}
</style>
