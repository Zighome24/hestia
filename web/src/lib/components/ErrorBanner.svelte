<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let message: string;

	const dispatch = createEventDispatcher();

	function dismiss() {
		dispatch('dismiss');
	}
</script>

{#if message}
	<div class="error-banner" role="alert">
		<span class="error-message">{message}</span>
		<button class="error-dismiss" on:click={dismiss} aria-label="Dismiss error">&times;</button>
	</div>
{/if}

<style>
	.error-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: var(--color-error-bg);
		border: 1px solid var(--color-error-border);
		border-radius: 0.5rem;
		margin-bottom: 1rem;
	}

	.error-message {
		color: var(--color-error-text);
		font-size: 0.9rem;
		line-height: 1.4;
	}

	.error-dismiss {
		background: none;
		border: none;
		color: var(--color-error-text);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0.25rem 0.5rem;
		line-height: 1;
		border-radius: 0.25rem;
		flex-shrink: 0;
	}

	.error-dismiss:hover {
		background: var(--color-error-hover);
	}
</style>
