<script lang="ts">
	import { login } from '$lib/auth';
	import { goto } from '$app/navigation';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	let username = '';
	let password = '';
	let error = '';
	let loading = false;

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';
		loading = true;
		try {
			await login(username, password);
			goto('/');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Login failed';
		} finally {
			loading = false;
		}
	}
</script>

<div class="page-center">
	<h1>Login</h1>

	{#if error}
		<ErrorBanner message={error} on:dismiss={() => (error = '')} />
	{/if}

	<form class="login-form" on:submit={handleSubmit}>
		<div class="field">
			<label for="username">Username</label>
			<input
				id="username"
				type="text"
				bind:value={username}
				autocomplete="username"
				required
			/>
		</div>

		<div class="field">
			<label for="password">Password</label>
			<input
				id="password"
				type="password"
				bind:value={password}
				autocomplete="current-password"
				required
			/>
		</div>

		<button type="submit" class="btn-primary" disabled={loading}>
			{#if loading}
				Signing in...
			{:else}
				Sign In
			{/if}
		</button>

		<p class="register-link">
			Don't have an account? <a href="/register">Register</a>
		</p>
	</form>
</div>

<style>
	.page-center {
		max-width: 24rem;
		margin: 0 auto;
	}

	h1 {
		margin-bottom: 1.5rem;
	}

	.login-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	input {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 1rem;
		width: 100%;
		min-height: 2.75rem;
	}

	input:focus {
		outline: none;
		border-color: #2563eb;
		box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.2);
	}

	.btn-primary {
		margin-top: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: #2563eb;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		min-height: 2.75rem;
		justify-content: center;
	}

	.btn-primary:hover {
		background: #1d4ed8;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.register-link {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
	}

	@media (max-width: 640px) {
		.page-center {
			max-width: 100%;
		}
	}
</style>
