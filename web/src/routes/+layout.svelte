<script lang="ts">
	import { onMount } from 'svelte';
	import { currentUser, checkAuth, logout } from '$lib/auth';
	import { goto } from '$app/navigation';

	let menuOpen = false;

	onMount(() => {
		checkAuth();
	});

	async function handleLogout() {
		await logout();
		menuOpen = false;
		goto('/login');
	}

	function closeMenu() {
		menuOpen = false;
	}
</script>

<div class="app">
	<nav class="nav">
		<div class="nav-top">
			<a href="/" class="nav-brand">Hestia</a>
			<button
				class="nav-toggle"
				on:click={() => (menuOpen = !menuOpen)}
				aria-label="Toggle navigation"
				aria-expanded={menuOpen}
			>
				<span class="hamburger" class:open={menuOpen}></span>
			</button>
		</div>
		<ul class="nav-links" class:open={menuOpen}>
			{#if $currentUser}
				<li><a href="/" on:click={closeMenu}>Home</a></li>
				<li><a href="/receipts" on:click={closeMenu}>Receipts</a></li>
				<li><a href="/settings/cards" on:click={closeMenu}>Settings</a></li>
				<li><a href="/settings/categories" on:click={closeMenu}>Categories</a></li>
				<li>
					<button class="nav-logout" on:click={handleLogout}>Logout</button>
				</li>
			{:else}
				<li><a href="/login" on:click={closeMenu}>Login</a></li>
				<li><a href="/register" on:click={closeMenu}>Register</a></li>
			{/if}
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

	/* Touch-friendly tap targets globally */
	:global(button),
	:global(a) {
		min-height: 2.75rem;
		display: inline-flex;
		align-items: center;
	}

	/* Reset min-height for inline text links */
	:global(p a),
	:global(span a),
	:global(.register-link a),
	:global(.login-link a) {
		min-height: unset;
		display: inline;
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
		padding: 0 1rem;
		background: #1e3a5f;
		color: white;
		min-height: 3.25rem;
	}

	.nav-top {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.nav-brand {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		letter-spacing: 0.02em;
		min-height: 3.25rem;
	}

	.nav-brand:hover {
		text-decoration: none;
		color: #93c5fd;
	}

	.nav-toggle {
		display: none;
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5rem;
		min-height: 2.75rem;
		min-width: 2.75rem;
		align-items: center;
		justify-content: center;
	}

	.hamburger {
		display: block;
		width: 1.25rem;
		height: 2px;
		background: white;
		position: relative;
		transition: background 0.2s;
	}

	.hamburger::before,
	.hamburger::after {
		content: '';
		display: block;
		width: 1.25rem;
		height: 2px;
		background: white;
		position: absolute;
		left: 0;
		transition: transform 0.2s;
	}

	.hamburger::before {
		top: -6px;
	}

	.hamburger::after {
		top: 6px;
	}

	.hamburger.open {
		background: transparent;
	}

	.hamburger.open::before {
		top: 0;
		transform: rotate(45deg);
	}

	.hamburger.open::after {
		top: 0;
		transform: rotate(-45deg);
	}

	.nav-links {
		display: flex;
		list-style: none;
		gap: 1.25rem;
		align-items: center;
	}

	.nav-links a {
		color: #bfdbfe;
		font-size: 0.95rem;
	}

	.nav-links a:hover {
		color: white;
		text-decoration: none;
	}

	.nav-logout {
		background: none;
		border: none;
		color: #bfdbfe;
		font-size: 0.95rem;
		cursor: pointer;
		padding: 0;
	}

	.nav-logout:hover {
		color: white;
	}

	.main {
		flex: 1;
		padding: 1.5rem 1rem;
		max-width: 48rem;
		width: 100%;
		margin: 0 auto;
	}

	@media (max-width: 640px) {
		.nav {
			flex-direction: column;
			align-items: stretch;
			padding: 0 1rem;
		}

		.nav-top {
			width: 100%;
			min-height: 3.25rem;
		}

		.nav-toggle {
			display: flex;
		}

		.nav-links {
			display: none;
			flex-direction: column;
			gap: 0;
			padding-bottom: 0.75rem;
		}

		.nav-links.open {
			display: flex;
		}

		.nav-links li {
			width: 100%;
		}

		.nav-links a,
		.nav-logout {
			display: flex;
			align-items: center;
			width: 100%;
			padding: 0.625rem 0;
			min-height: 2.75rem;
			font-size: 1rem;
		}

		.main {
			padding: 1rem 0.75rem;
		}
	}
</style>
