<script lang="ts">
	import { onMount } from 'svelte';
	import { currentUser, checkAuth, logout } from '$lib/auth';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { theme, toggleTheme } from '$lib/theme';

	const publicPaths = ['/login', '/register'];

	let menuOpen = false;
	let authChecked = false;

	onMount(async () => {
		const user = await checkAuth();
		authChecked = true;
		if (!user && !publicPaths.includes(window.location.pathname)) {
			goto('/login');
		}
	});

	// Redirect on subsequent navigations if not authenticated
	$: if (authChecked && !$currentUser && !publicPaths.includes($page.url.pathname)) {
		goto('/login');
	}

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
			<div class="nav-top-right">
				<button
					class="theme-toggle"
					on:click={toggleTheme}
					aria-label="Toggle dark mode"
					title={$theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
				>
					{#if $theme === 'light'}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
					{:else}
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
					{/if}
				</button>
				<button
					class="nav-toggle"
					on:click={() => (menuOpen = !menuOpen)}
					aria-label="Toggle navigation"
					aria-expanded={menuOpen}
				>
					<span class="hamburger" class:open={menuOpen}></span>
				</button>
			</div>
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
	:global(:root) {
		--color-bg: #f5f5f5;
		--color-bg-card: #ffffff;
		--color-bg-card-hover: #f9fafb;
		--color-bg-secondary: #f3f4f6;
		--color-bg-secondary-hover: #e5e7eb;
		--color-text: #1a1a1a;
		--color-text-secondary: #6b7280;
		--color-text-label: #374151;
		--color-text-heading: #111827;
		--color-border: #e5e7eb;
		--color-border-input: #d1d5db;
		--color-primary: #2563eb;
		--color-primary-hover: #1d4ed8;
		--color-primary-light: #eff6ff;
		--color-primary-badge: #dbeafe;
		--color-link: #2563eb;
		--color-nav-bg: #1e3a5f;
		--color-nav-text: #bfdbfe;
		--color-nav-brand-hover: #93c5fd;
		--color-heading-primary: #1e3a5f;
		--color-error-bg: #fef2f2;
		--color-error-border: #fca5a5;
		--color-error-text: #b91c1c;
		--color-error-hover: #fee2e2;
		--color-danger-bg: #fee2e2;
		--color-danger-text: #dc2626;
		--color-danger-border: #fca5a5;
		--color-danger-hover: #fecaca;
		--color-spinner-border: #e5e7eb;
		--color-focus-shadow: rgba(37, 99, 235, 0.2);
	}

	:global([data-theme='dark']) {
		--color-bg: #111827;
		--color-bg-card: #1f2937;
		--color-bg-card-hover: #283548;
		--color-bg-secondary: #374151;
		--color-bg-secondary-hover: #4b5563;
		--color-text: #e5e7eb;
		--color-text-secondary: #9ca3af;
		--color-text-label: #d1d5db;
		--color-text-heading: #f3f4f6;
		--color-border: #374151;
		--color-border-input: #4b5563;
		--color-primary: #3b82f6;
		--color-primary-hover: #2563eb;
		--color-primary-light: #1e293b;
		--color-primary-badge: #1e3a5f;
		--color-link: #60a5fa;
		--color-nav-bg: #0f172a;
		--color-nav-text: #94a3b8;
		--color-nav-brand-hover: #93c5fd;
		--color-heading-primary: #60a5fa;
		--color-error-bg: #451a1a;
		--color-error-border: #991b1b;
		--color-error-text: #fca5a5;
		--color-error-hover: #5c2020;
		--color-danger-bg: #451a1a;
		--color-danger-text: #f87171;
		--color-danger-border: #991b1b;
		--color-danger-hover: #5c2020;
		--color-spinner-border: #374151;
		--color-focus-shadow: rgba(59, 130, 246, 0.3);
	}

	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
		font-size: 16px;
		line-height: 1.5;
		color: var(--color-text);
		background: var(--color-bg);
	}

	:global(a) {
		color: var(--color-link);
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
		background: var(--color-nav-bg);
		color: white;
		min-height: 3.25rem;
	}

	.nav-top {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.nav-top-right {
		display: flex;
		align-items: center;
		gap: 0.25rem;
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
		color: var(--color-nav-brand-hover);
	}

	.theme-toggle {
		background: none;
		border: none;
		color: var(--color-nav-text);
		cursor: pointer;
		padding: 0.4rem;
		min-height: 2.25rem;
		min-width: 2.25rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.375rem;
		transition: color 0.15s;
	}

	.theme-toggle:hover {
		color: white;
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
		color: var(--color-nav-text);
		font-size: 0.95rem;
	}

	.nav-links a:hover {
		color: white;
		text-decoration: none;
	}

	.nav-logout {
		background: none;
		border: none;
		color: var(--color-nav-text);
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
