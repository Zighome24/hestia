<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { get, del } from '$lib/api';
	import { goto } from '$app/navigation';

	interface Category { id: string; name: string; color: string; }
	interface Receipt {
		id: string; user_id: string; card_id: string | null;
		total_amount: number; photo_path: string | null;
		notes: string | null; purchased_at: string; created_at: string;
		categories: Category[];
	}
	interface Card { id: string; nickname: string; last_four: string; }

	let receipt: Receipt | null = null;
	let cards: Card[] = [];
	let loading = true;
	let error = '';

	$: receiptId = $page.params.id;

	onMount(async () => {
		try {
			[receipt, cards] = await Promise.all([
				get<Receipt>(`/receipts/${receiptId}`),
				get<Card[]>('/cards')
			]);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load receipt';
		} finally {
			loading = false;
		}
	});

	function formatAmount(cents: number): string {
		return `$${(cents / 100).toFixed(2)}`;
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });
	}

	function cardName(cardId: string | null): string {
		if (!cardId) return 'None';
		const card = cards.find(c => c.id === cardId);
		return card ? `${card.nickname} (${card.last_four})` : 'Unknown';
	}

	async function handleDelete() {
		if (!confirm('Delete this receipt?')) return;
		try {
			await del(`/receipts/${receiptId}`);
			goto('/receipts');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete';
		}
	}
</script>

{#if loading}
	<p>Loading...</p>
{:else if error}
	<p class="error" role="alert">{error}</p>
{:else if receipt}
	<div class="detail-header">
		<h1>{formatAmount(receipt.total_amount)}</h1>
		<button class="btn-danger" on:click={handleDelete}>Delete</button>
	</div>

	{#if receipt.photo_path}
		<div class="photo-section">
			<img src="/api/receipts/{receipt.id}/photo" alt="Receipt photo" class="receipt-photo" />
		</div>
	{/if}

	<div class="detail-fields">
		<div class="detail-row">
			<span class="detail-label">Date</span>
			<span class="detail-value">{formatDate(receipt.purchased_at)}</span>
		</div>
		<div class="detail-row">
			<span class="detail-label">Card</span>
			<span class="detail-value">{cardName(receipt.card_id)}</span>
		</div>
		{#if receipt.categories.length > 0}
			<div class="detail-row">
				<span class="detail-label">Categories</span>
				<div class="categories">
					{#each receipt.categories as cat}
						<span class="category-badge" style="background: {cat.color}20; color: {cat.color}; border: 1px solid {cat.color}40">
							{cat.name}
						</span>
					{/each}
				</div>
			</div>
		{/if}
		{#if receipt.notes}
			<div class="detail-row">
				<span class="detail-label">Notes</span>
				<span class="detail-value">{receipt.notes}</span>
			</div>
		{/if}
	</div>

	<a href="/receipts" class="back-link">Back to receipts</a>
{/if}

<style>
	.detail-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; }
	h1 { font-size: 2rem; }
	.photo-section { margin-bottom: 1.5rem; }
	.receipt-photo { max-width: 100%; max-height: 400px; border-radius: 0.5rem; border: 1px solid #e5e7eb; }
	.detail-fields { display: flex; flex-direction: column; gap: 0.75rem; }
	.detail-row { display: flex; gap: 1rem; align-items: flex-start; }
	.detail-label { font-size: 0.85rem; font-weight: 500; color: #6b7280; min-width: 6rem; }
	.detail-value { color: #111827; }
	.categories { display: flex; gap: 0.35rem; flex-wrap: wrap; }
	.category-badge { font-size: 0.75rem; padding: 0.1rem 0.5rem; border-radius: 1rem; }
	.btn-danger { padding: 0.4rem 0.8rem; background: #fee2e2; color: #dc2626; border: 1px solid #fca5a5; border-radius: 0.375rem; cursor: pointer; font-size: 0.9rem; min-height: 2.75rem; }
	.btn-danger:hover { background: #fecaca; }
	.back-link { display: inline-block; margin-top: 1.5rem; font-size: 0.9rem; }
	.error { color: #dc2626; }

	@media (max-width: 640px) {
		.detail-row { flex-direction: column; gap: 0.25rem; }
		.detail-label { min-width: unset; }
		.receipt-photo { max-height: none; }
	}
</style>
