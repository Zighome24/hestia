<script lang="ts">
	import { register } from '$lib/auth';
	import { goto } from '$app/navigation';

	let username = '';
	let displayName = '';
	let password = '';
	let confirmPassword = '';
	let error = '';
	let loading = false;

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			return;
		}
		if (password.length < 8) {
			error = 'Password must be at least 8 characters';
			return;
		}

		loading = true;
		try {
			await register(username, displayName, password);
			goto('/');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Registration failed';
		} finally {
			loading = false;
		}
	}
</script>

<h1>Register</h1>

{#if error}
	<p class="error" role="alert">{error}</p>
{/if}

<form class="register-form" on:submit={handleSubmit}>
	<div class="field">
		<label for="username">Username</label>
		<input id="username" type="text" bind:value={username} autocomplete="username" required />
	</div>

	<div class="field">
		<label for="display-name">Display Name</label>
		<input id="display-name" type="text" bind:value={displayName} required />
	</div>

	<div class="field">
		<label for="password">Password</label>
		<input
			id="password"
			type="password"
			bind:value={password}
			autocomplete="new-password"
			required
		/>
	</div>

	<div class="field">
		<label for="confirm-password">Confirm Password</label>
		<input
			id="confirm-password"
			type="password"
			bind:value={confirmPassword}
			autocomplete="new-password"
			required
		/>
	</div>

	<button type="submit" class="btn-primary" disabled={loading}>
		{loading ? 'Creating account...' : 'Create Account'}
	</button>

	<p class="login-link">
		Already have an account? <a href="/login">Sign in</a>
	</p>
</form>

<style>
	h1 {
		margin-bottom: 1.5rem;
	}

	.register-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		max-width: 24rem;
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
	}

	.btn-primary:hover {
		background: #1d4ed8;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.error {
		color: #dc2626;
		margin-bottom: 0.5rem;
	}

	.login-link {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		color: #6b7280;
	}
</style>
