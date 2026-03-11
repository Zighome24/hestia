<script lang="ts">
	import { onMount } from 'svelte';
	import { get, post } from '$lib/api';
	import { goto } from '$app/navigation';
	import Spinner from '$lib/components/Spinner.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	interface Category { id: string; name: string; color: string; }
	interface Card { id: string; nickname: string; last_four: string; }
	interface Receipt { id: string; }

	let cards: Card[] = [];
	let categories: Category[] = [];
	let loading = true;

	let cardId = '';
	let amount = '';
	let purchasedAt = new Date().toISOString().slice(0, 10);
	let notes = '';
	let selectedCategories: string[] = [];
	let photoFile: File | null = null;
	let error = '';
	let submitting = false;

	onMount(async () => {
		try {
			[cards, categories] = await Promise.all([
				get<Card[]>('/cards'),
				get<Category[]>('/categories')
			]);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load';
		} finally {
			loading = false;
		}
	});

	function handleFileChange(e: Event) {
		const input = e.target as HTMLInputElement;
		photoFile = input.files?.[0] ?? null;
	}

	function toggleCategory(id: string) {
		if (selectedCategories.includes(id)) {
			selectedCategories = selectedCategories.filter(c => c !== id);
		} else {
			selectedCategories = [...selectedCategories, id];
		}
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		error = '';

		const amountNum = parseFloat(amount);
		if (isNaN(amountNum) || amountNum < 0) {
			error = 'Please enter a valid amount';
			return;
		}
		const totalCents = Math.round(amountNum * 100);

		submitting = true;
		try {
			const receipt = await post<Receipt>('/receipts', {
				card_id: cardId || null,
				total_amount: totalCents,
				notes: notes || null,
				purchased_at: new Date(purchasedAt).toISOString(),
				category_ids: selectedCategories.length > 0 ? selectedCategories : null
			});

			// Upload photo if selected
			if (photoFile) {
				const formData = new FormData();
				formData.append('file', photoFile);
				await fetch(`/api/receipts/${receipt.id}/photo`, {
					method: 'POST',
					credentials: 'include',
					body: formData
				});
			}

			goto(`/receipts/${receipt.id}`);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to create receipt';
		} finally {
			submitting = false;
		}
	}
</script>

<h1>Add Receipt</h1>

{#if error}
	<ErrorBanner message={error} on:dismiss={() => (error = '')} />
{/if}

{#if loading}
	<Spinner message="Loading form data..." />
{:else}
	<form class="receipt-form" on:submit={handleSubmit}>
		<div class="field">
			<label for="amount">Amount ($)</label>
			<input id="amount" type="number" step="0.01" min="0" bind:value={amount} placeholder="0.00" required />
		</div>

		<div class="field">
			<label for="purchased-at">Date</label>
			<input id="purchased-at" type="date" bind:value={purchasedAt} required />
		</div>

		<div class="field">
			<label for="card">Card</label>
			<select id="card" bind:value={cardId}>
				<option value="">No card</option>
				{#each cards as card}
					<option value={card.id}>{card.nickname} ({card.last_four})</option>
				{/each}
			</select>
		</div>

		<div class="field">
			<label for="photo">Photo</label>
			<input id="photo" type="file" accept="image/jpeg,image/png" capture="environment" on:change={handleFileChange} />
		</div>

		{#if categories.length > 0}
			<div class="field">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<label>Categories</label>
				<div class="category-picks" role="group" aria-label="Categories">
					{#each categories as cat}
						<button
							type="button"
							class="category-pick"
							class:selected={selectedCategories.includes(cat.id)}
							style="--cat-color: {cat.color}"
							on:click={() => toggleCategory(cat.id)}
						>
							{cat.name}
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<div class="field">
			<label for="notes">Notes</label>
			<textarea id="notes" bind:value={notes} rows="3" placeholder="Optional notes..."></textarea>
		</div>

		<button type="submit" class="btn-primary" disabled={submitting || loading}>
			{submitting ? 'Saving...' : 'Save Receipt'}
		</button>
	</form>
{/if}

<style>
	h1 { margin-bottom: 1.5rem; }
	.receipt-form { display: flex; flex-direction: column; gap: 1rem; max-width: 32rem; }
	.field { display: flex; flex-direction: column; gap: 0.25rem; }
	label { font-size: 0.875rem; font-weight: 500; color: var(--color-text-label); }
	input, select, textarea {
		padding: 0.5rem 0.75rem; border: 1px solid var(--color-border-input); border-radius: 0.375rem; font-size: 1rem; width: 100%; min-height: 2.5rem;
		background: var(--color-bg-card); color: var(--color-text);
	}
	input:focus, select:focus, textarea:focus {
		outline: none; border-color: var(--color-primary); box-shadow: 0 0 0 2px var(--color-focus-shadow);
	}
	input[type="file"] { padding: 0.375rem; }
	.category-picks { display: flex; gap: 0.4rem; flex-wrap: wrap; }
	.category-pick {
		padding: 0.3rem 0.7rem; border-radius: 1rem; font-size: 0.85rem; cursor: pointer;
		background: var(--color-bg-secondary); color: var(--color-text-label); border: 1px solid var(--color-border-input); transition: all 0.15s;
	}
	.category-pick.selected {
		background: var(--cat-color, var(--color-primary)); color: white; border-color: var(--cat-color, var(--color-primary));
	}
	.btn-primary {
		margin-top: 0.5rem; padding: 0.625rem 1.25rem; background: var(--color-primary); color: white;
		border: none; border-radius: 0.375rem; font-size: 1rem; font-weight: 500; cursor: pointer;
		min-height: 2.75rem; justify-content: center;
	}
	.btn-primary:hover { background: var(--color-primary-hover); }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
	@media (max-width: 640px) {
		.receipt-form { max-width: 100%; }
	}
</style>
