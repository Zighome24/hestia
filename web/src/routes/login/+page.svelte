<script lang="ts">
	let username = '';
	let password = '';

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		// TODO: implement login
	}
</script>

<h1>Login</h1>

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

	<button type="submit" class="btn-primary">Sign In</button>
</form>

<style>
	h1 {
		margin-bottom: 1.5rem;
	}

	.login-form {
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
</style>
