<script lang="ts">
	import { onMount } from 'svelte';
	import { get } from '$lib/api';
	import Spinner from '$lib/components/Spinner.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	interface Category { id: string; name: string; color: string; created_at: string; }
	interface Card { id: string; user_id: string; nickname: string; last_four: string; created_at: string; }
	interface Receipt {
		id: string; user_id: string; card_id: string | null;
		total_amount: number; photo_path: string | null;
		notes: string | null; purchased_at: string; created_at: string;
		categories: Category[];
	}

	let receipts: Receipt[] = [];
	let cards: Card[] = [];
	let categories: Category[] = [];
	let loading = true;
	let error = '';

	let filterCard = '';
	let filterCategory = '';
	let sortNewest = true;

	onMount(async () => {
		try {
			[receipts, cards, categories] = await Promise.all([
				get<Receipt[]>('/receipts'),
				get<Card[]>('/cards'),
				get<Category[]>('/categories')
			]);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load';
		} finally {
			loading = false;
		}
	});

	function formatAmount(cents: number): string {
		return `$${(cents / 100).toFixed(2)}`;
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString();
	}

	function cardName(cardId: string | null): string {
		if (!cardId) return '';
		const card = cards.find(c => c.id === cardId);
		return card ? `${card.nickname} (${card.last_four})` : '';
	}

	$: filtered = receipts
		.filter(r => !filterCard || r.card_id === filterCard)
		.filter(r => !filterCategory || r.categories.some(c => c.id === filterCategory))
		.sort((a, b) => {
			const da = new Date(a.purchased_at).getTime();
			const db = new Date(b.purchased_at).getTime();
			return sortNewest ? db - da : da - db;
		});
</script>

<div class="header">
	<h1>Receipts</h1>
	<a href="/receipts/new" class="btn-primary">Add Receipt</a>
</div>

{#if error}
	<ErrorBanner message={error} on:dismiss={() => (error = '')} />
{/if}

{#if loading}
	<Spinner message="Loading receipts..." />
{:else}
	<div class="filters">
		<select bind:value={filterCard} aria-label="Filter by card">
			<option value="">All Cards</option>
			{#each cards as card}
				<option value={card.id}>{card.nickname} ({card.last_four})</option>
			{/each}
		</select>
		<select bind:value={filterCategory} aria-label="Filter by category">
			<option value="">All Categories</option>
			{#each categories as cat}
				<option value={cat.id}>{cat.name}</option>
			{/each}
		</select>
		<button class="btn-small btn-secondary" on:click={() => sortNewest = !sortNewest}>
			{sortNewest ? 'Newest first' : 'Oldest first'}
		</button>
	</div>

	{#if filtered.length === 0}
		<p class="empty">No receipts found.</p>
	{:else}
		<ul class="receipt-list">
			{#each filtered as receipt (receipt.id)}
				<li class="receipt-item">
					<a href="/receipts/{receipt.id}" class="receipt-link">
						<div class="receipt-main">
							<span class="receipt-date">{formatDate(receipt.purchased_at)}</span>
							<span class="receipt-amount">{formatAmount(receipt.total_amount)}</span>
						</div>
						<div class="receipt-meta">
							{#if receipt.card_id}
								<span class="receipt-card">{cardName(receipt.card_id)}</span>
							{/if}
							{#if receipt.photo_path}
								<span class="receipt-photo-badge">Photo</span>
							{/if}
						</div>
						{#if receipt.categories.length > 0}
							<div class="receipt-categories">
								{#each receipt.categories as cat}
									<span class="category-badge" style="background: {cat.color}20; color: {cat.color}; border: 1px solid {cat.color}40">
										{cat.name}
									</span>
								{/each}
							</div>
						{/if}
						{#if receipt.notes}
							<p class="receipt-notes">{receipt.notes}</p>
						{/if}
					</a>
				</li>
			{/each}
		</ul>
	{/if}
{/if}

<style>
	.header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem; gap: 0.75rem; flex-wrap: wrap; }
	.btn-primary { padding: 0.5rem 1rem; background: var(--color-primary); color: white; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 500; min-height: 2.75rem; }
	.btn-primary:hover { background: var(--color-primary-hover); text-decoration: none; }
	.filters { display: flex; gap: 0.75rem; margin-bottom: 1rem; flex-wrap: wrap; }
	select { padding: 0.4rem 0.6rem; border: 1px solid var(--color-border-input); border-radius: 0.375rem; font-size: 0.9rem; min-height: 2.5rem; background: var(--color-bg-card); color: var(--color-text); }
	.btn-small { padding: 0.4rem 0.75rem; font-size: 0.85rem; border-radius: 0.3rem; cursor: pointer; min-height: 2.75rem; }
	.btn-secondary { background: var(--color-bg-secondary); color: var(--color-text-label); border: 1px solid var(--color-border-input); }
	.btn-secondary:hover { background: var(--color-bg-secondary-hover); }
	.receipt-list { list-style: none; display: flex; flex-direction: column; gap: 0.5rem; }
	.receipt-item { background: var(--color-bg-card); border: 1px solid var(--color-border); border-radius: 0.5rem; }
	.receipt-link { display: block; padding: 0.75rem 1rem; color: inherit; text-decoration: none; min-height: unset; }
	.receipt-link:hover { background: var(--color-bg-card-hover); text-decoration: none; }
	.receipt-main { display: flex; justify-content: space-between; align-items: center; }
	.receipt-date { font-size: 0.9rem; color: var(--color-text-label); }
	.receipt-amount { font-size: 1.1rem; font-weight: 600; color: var(--color-text-heading); }
	.receipt-meta { display: flex; gap: 0.5rem; margin-top: 0.25rem; }
	.receipt-card { font-size: 0.8rem; color: var(--color-text-secondary); }
	.receipt-photo-badge { font-size: 0.75rem; background: var(--color-primary-badge); color: var(--color-primary); padding: 0.1rem 0.4rem; border-radius: 0.25rem; }
	.receipt-categories { display: flex; gap: 0.35rem; margin-top: 0.35rem; flex-wrap: wrap; }
	.category-badge { font-size: 0.75rem; padding: 0.1rem 0.5rem; border-radius: 1rem; }
	.receipt-notes { font-size: 0.8rem; color: var(--color-text-secondary); margin-top: 0.25rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
	.empty { color: var(--color-text-secondary); text-align: center; padding: 2rem 0; }

	@media (max-width: 640px) {
		.filters { flex-direction: column; }
		select, .btn-small { width: 100%; }
	}
</style>
