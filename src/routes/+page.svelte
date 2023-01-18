<script>
	import Icon from '@iconify/svelte/dist/OfflineIcon.svelte';
	import dotMark from '@iconify-icons/carbon/dot-mark';
	import github from '@iconify-icons/carbon/logo-github';
	import linkedin from '@iconify-icons/carbon/logo-linkedin';
	import email from '@iconify-icons/carbon/email';
</script>

<svelte:head>
	<title>Luís Fonseca</title>
	<meta name="description" content="Luís Fonseca's personal website" />
	<meta name="author" content="Luís Fonseca" />
</svelte:head>

<div id="container">
	<div id="links">
		<a href="/about"><span>About</span></a>
		<span><Icon icon={dotMark} /></span>
		<a href="/cv"><span>CV</span></a>
		<span><Icon icon={dotMark} /></span>
		<a href="/posts"><span>Posts</span></a>
	</div>
	<div id="social">
		<a aria-label="Github" href="https://github.com/luishfonseca">
			<Icon height="64px" icon={github} />
		</a>
		<a aria-label="Email" href="mailto:luis@lhf.pt" target="_blank">
			<Icon height="64px" icon={email} />
		</a>
		<a aria-label="Linkedin" href="https://www.linkedin.com/in/luishfonseca/">
			<Icon height="64px" icon={linkedin} />
		</a>
	</div>
</div>

<style lang="scss">
	#container {
		display: flex;
		height: 100%;
		width: 100%;
		flex-direction: column;
		justify-content: center;
	}

	#links {
		display: flex;
		justify-content: center;
		margin-bottom: 20px;
	}

	#links > a {
		transition: all 0.2s ease-in-out;
		text-decoration: none;
		font-size: 2.5em;
		font-weight: 200;
		margin: 0 10px;
	}

	#links > :last-child,
	#links > :first-child {
		flex-grow: 1;
		flex-basis: 0;
	}

	#links > :first-child {
		text-align: right;
	}

	#links > a > span {
		border-bottom: dotted 5px transparent;
		color: var(--subtle-color);
		padding: 0 2px;
	}

	#links > a:hover > span,
	#links > a:focus > span {
		color: var(--purple);
		border-bottom: dotted 5px;
	}

	#links > span {
		color: var(--muted-color);
		display: flex;
		align-items: center;
	}

	#social {
		display: flex;
		justify-content: center;
		margin-top: 20px;
	}

	#social > a {
		color: var(--subtle-color);
		margin: 0 10px;
	}

	#social > a:hover,
	#social > a:focus {
		color: var(--purple);
		transition: color 0.2s ease-in-out;
	}

	@media (max-width: 768px) {
		#links > * {
			text-align: center !important;
		}

		#links {
			flex-direction: column;
		}

		#links > span {
			visibility: hidden;
		}
	}
</style>
