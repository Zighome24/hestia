<script lang="ts">
	import { currentUser } from '$lib/auth';
</script>

<div class="home">
	<h1>Welcome to Hestia</h1>
	<p class="tagline">Your family's home base. Track receipts, manage expenses, and stay organized.</p>

	{#if $currentUser}
		<div class="quick-links">
			<a href="/receipts" class="quick-link">
				<span class="quick-link-icon">&#128451;</span>
				<span class="quick-link-label">Receipts</span>
				<span class="quick-link-desc">View and manage your receipts</span>
			</a>
			<a href="/receipts/new" class="quick-link">
				<span class="quick-link-icon">&#10133;</span>
				<span class="quick-link-label">Add Receipt</span>
				<span class="quick-link-desc">Upload a new receipt</span>
			</a>
			<a href="/settings/cards" class="quick-link">
				<span class="quick-link-icon">&#128179;</span>
				<span class="quick-link-label">Cards</span>
				<span class="quick-link-desc">Manage your credit cards</span>
			</a>
		</div>
	{:else}
		<div class="cta">
			<a href="/login" class="btn-primary">Sign In</a>
			<a href="/register" class="btn-secondary">Create Account</a>
		</div>
	{/if}
</div>

<style>
	.home {
		text-align: center;
		padding: 2rem 0;
	}

	h1 {
		font-size: 2rem;
		margin-bottom: 0.5rem;
		color: #1e3a5f;
	}

	.tagline {
		color: #6b7280;
		font-size: 1.1rem;
		margin-bottom: 2.5rem;
	}

	.quick-links {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		max-width: 36rem;
		margin: 0 auto;
	}

	.quick-link {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1.5rem 1rem;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		color: #1a1a1a;
		transition: border-color 0.15s, box-shadow 0.15s;
		min-height: 2.75rem;
	}

	.quick-link:hover {
		text-decoration: none;
		border-color: #2563eb;
		box-shadow: 0 2px 8px rgba(37, 99, 235, 0.1);
	}

	.quick-link-icon {
		font-size: 2rem;
		line-height: 1;
	}

	.quick-link-label {
		font-weight: 600;
		font-size: 1rem;
	}

	.quick-link-desc {
		font-size: 0.8rem;
		color: #6b7280;
	}

	.cta {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn-primary,
	.btn-secondary {
		padding: 0.75rem 2rem;
		border-radius: 0.5rem;
		font-size: 1rem;
		font-weight: 500;
		min-height: 2.75rem;
		display: inline-flex;
		align-items: center;
	}

	.btn-primary {
		background: #2563eb;
		color: white;
	}

	.btn-primary:hover {
		background: #1d4ed8;
		text-decoration: none;
	}

	.btn-secondary {
		background: white;
		color: #2563eb;
		border: 1px solid #2563eb;
	}

	.btn-secondary:hover {
		background: #eff6ff;
		text-decoration: none;
	}

	@media (max-width: 640px) {
		h1 {
			font-size: 1.5rem;
		}

		.tagline {
			font-size: 1rem;
		}

		.quick-links {
			grid-template-columns: 1fr;
			max-width: 100%;
		}

		.quick-link {
			flex-direction: row;
			text-align: left;
			padding: 1rem;
		}

		.quick-link-icon {
			font-size: 1.5rem;
		}
	}
</style>
