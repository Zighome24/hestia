<script lang="ts">
</script>

<div class="app">
	<nav class="nav">
		<a href="/" class="nav-brand">Hestia</a>
		<ul class="nav-links">
			<li><a href="/">Home</a></li>
			<li><a href="/receipts">Receipts</a></li>
			<li><a href="/settings/cards">Settings</a></li>
		</ul>
	</nav>

	<main class="main">
		<slot />
	</main>
</div>

<style>
	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
		font-size: 16px;
		line-height: 1.5;
		color: #1a1a1a;
		background: #f5f5f5;
	}

	:global(a) {
		color: #2563eb;
		text-decoration: none;
	}

	:global(a:hover) {
		text-decoration: underline;
	}

	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	.nav {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: #1e3a5f;
		color: white;
	}

	.nav-brand {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		letter-spacing: 0.02em;
	}

	.nav-brand:hover {
		text-decoration: none;
		color: #93c5fd;
	}

	.nav-links {
		display: flex;
		list-style: none;
		gap: 1.25rem;
	}

	.nav-links a {
		color: #bfdbfe;
		font-size: 0.95rem;
	}

	.nav-links a:hover {
		color: white;
		text-decoration: none;
	}

	.main {
		flex: 1;
		padding: 1.5rem 1rem;
		max-width: 48rem;
		width: 100%;
		margin: 0 auto;
	}
</style>
