<script lang="ts">
	export let size: string = '1.5rem';
	export let message: string = 'Loading...';
</script>

<div class="spinner-wrapper" role="status" aria-label={message}>
	<div class="spinner" style="width: {size}; height: {size};"></div>
	{#if message}
		<span class="spinner-text">{message}</span>
	{/if}
</div>

<style>
	.spinner-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 0;
	}

	.spinner {
		border: 3px solid #e5e7eb;
		border-top-color: #2563eb;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
	}

	.spinner-text {
		color: #6b7280;
		font-size: 0.9rem;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
