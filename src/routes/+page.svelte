<script lang="ts">
	let year = new Date().getFullYear();
</script>

<div
	flex
	absolute
	bottom-0
	left-0
	min-h-screen
	min-w-screen
	text-center
	select-none
	all:transition-400
>
	<div ma>
		<div text-5xl fw100 animate-bounce-alt animate-count-infinite animate-duration-2s>
			Luís Fonseca
		</div>
		<div op30 text-lg fw300 m1>Come and read my procrastination.</div>
		<div m2 flex justify-center text-2xl>
			<a text-inherit href="https://github.com/luishfonseca">
				<div i-carbon-logo-github op30 hover="op80" />
			</a>
		</div>
	</div>
</div>
<div absolute bottom-5 right-0 left-0 text-center op30 fw300>Copyright © {year} Luís Fonseca</div>
