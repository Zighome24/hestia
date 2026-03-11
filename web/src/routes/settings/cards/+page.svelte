<script lang="ts">
	import { onMount } from 'svelte';
	import { get, post, put, del } from '$lib/api';
	import Spinner from '$lib/components/Spinner.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	interface Card {
		id: string;
		user_id: string;
		nickname: string;
		last_four: string;
		created_at: string;
	}

	let cards: Card[] = [];
	let loading = true;
	let error = '';

	// Add form
	let newNickname = '';
	let newLastFour = '';
	let addError = '';
	let adding = false;

	// Edit state
	let editingId: string | null = null;
	let editNickname = '';
	let editLastFour = '';
	let editError = '';

	onMount(async () => {
		await loadCards();
	});

	async function loadCards() {
		loading = true;
		error = '';
		try {
			cards = await get<Card[]>('/cards');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load cards';
		} finally {
			loading = false;
		}
	}

	function validateLastFour(value: string): string | null {
		if (value.length !== 4) return 'Must be exactly 4 digits';
		if (!/^\d{4}$/.test(value)) return 'Must contain only digits';
		return null;
	}

	async function handleAdd(e: SubmitEvent) {
		e.preventDefault();
		addError = '';

		if (!newNickname.trim()) {
			addError = 'Nickname is required';
			return;
		}
		const lastFourErr = validateLastFour(newLastFour);
		if (lastFourErr) {
			addError = lastFourErr;
			return;
		}

		adding = true;
		try {
			const card = await post<Card>('/cards', {
				nickname: newNickname.trim(),
				last_four: newLastFour
			});
			cards = [card, ...cards];
			newNickname = '';
			newLastFour = '';
		} catch (err) {
			addError = err instanceof Error ? err.message : 'Failed to add card';
		} finally {
			adding = false;
		}
	}

	function startEdit(card: Card) {
		editingId = card.id;
		editNickname = card.nickname;
		editLastFour = card.last_four;
		editError = '';
	}

	function cancelEdit() {
		editingId = null;
		editError = '';
	}

	async function handleEdit(e: SubmitEvent) {
		e.preventDefault();
		editError = '';

		if (!editNickname.trim()) {
			editError = 'Nickname is required';
			return;
		}
		const lastFourErr = validateLastFour(editLastFour);
		if (lastFourErr) {
			editError = lastFourErr;
			return;
		}

		try {
			const updated = await put<Card>(`/cards/${editingId}`, {
				nickname: editNickname.trim(),
				last_four: editLastFour
			});
			cards = cards.map((c) => (c.id === editingId ? updated : c));
			editingId = null;
		} catch (err) {
			editError = err instanceof Error ? err.message : 'Failed to update card';
		}
	}

	async function handleDelete(id: string) {
		if (!confirm('Delete this card?')) return;
		try {
			await del(`/cards/${id}`);
			cards = cards.filter((c) => c.id !== id);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete card';
		}
	}
</script>

<h1>Credit Cards</h1>

{#if error}
	<ErrorBanner message={error} on:dismiss={() => (error = '')} />
{/if}

<section class="add-section">
	<h2>Add Card</h2>
	{#if addError}
		<ErrorBanner message={addError} on:dismiss={() => (addError = '')} />
	{/if}
	<form class="card-form" on:submit={handleAdd}>
		<div class="field">
			<label for="new-nickname">Nickname</label>
			<input id="new-nickname" type="text" bind:value={newNickname} placeholder="e.g. Chase Sapphire" required />
		</div>
		<div class="field">
			<label for="new-last-four">Last 4 Digits</label>
			<input id="new-last-four" type="text" bind:value={newLastFour} placeholder="1234" maxlength="4" pattern={"\\d{4}"} required />
		</div>
		<button type="submit" class="btn-primary" disabled={adding}>
			{adding ? 'Adding...' : 'Add Card'}
		</button>
	</form>
</section>

<section class="cards-section">
	<h2>Your Cards</h2>
	{#if loading}
		<Spinner message="Loading cards..." />
	{:else if cards.length === 0}
		<p class="empty">No cards yet. Add one above.</p>
	{:else}
		<ul class="card-list">
			{#each cards as card (card.id)}
				<li class="card-item">
					{#if editingId === card.id}
						{#if editError}
							<ErrorBanner message={editError} on:dismiss={() => (editError = '')} />
						{/if}
						<form class="card-form inline" on:submit={handleEdit}>
							<div class="field">
								<label for="edit-nickname">Nickname</label>
								<input id="edit-nickname" type="text" bind:value={editNickname} required />
							</div>
							<div class="field">
								<label for="edit-last-four">Last 4</label>
								<input id="edit-last-four" type="text" bind:value={editLastFour} maxlength="4" pattern={"\\d{4}"} required />
							</div>
							<div class="card-actions">
								<button type="submit" class="btn-small btn-primary">Save</button>
								<button type="button" class="btn-small btn-secondary" on:click={cancelEdit}>Cancel</button>
							</div>
						</form>
					{:else}
						<div class="card-info">
							<span class="card-nickname">{card.nickname}</span>
							<span class="card-digits">---- {card.last_four}</span>
						</div>
						<div class="card-actions">
							<button class="btn-small btn-secondary" on:click={() => startEdit(card)}>Edit</button>
							<button class="btn-small btn-danger" on:click={() => handleDelete(card.id)}>Delete</button>
						</div>
					{/if}
				</li>
			{/each}
		</ul>
	{/if}
</section>

<style>
	h1 { margin-bottom: 1.5rem; }
	h2 { margin-bottom: 0.75rem; font-size: 1.1rem; }

	.add-section { margin-bottom: 2rem; }
	.cards-section { margin-bottom: 2rem; }

	.card-form {
		display: flex;
		gap: 0.75rem;
		align-items: flex-end;
		flex-wrap: wrap;
	}

	.card-form.inline {
		width: 100%;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	label {
		font-size: 0.8rem;
		font-weight: 500;
		color: var(--color-text-label);
	}

	input {
		padding: 0.5rem 0.6rem;
		border: 1px solid var(--color-border-input);
		border-radius: 0.375rem;
		font-size: 0.95rem;
		min-height: 2.5rem;
		background: var(--color-bg-card);
		color: var(--color-text);
	}

	input:focus {
		outline: none;
		border-color: var(--color-primary);
		box-shadow: 0 0 0 2px var(--color-focus-shadow);
	}

	.card-list {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.card-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.card-info {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.card-nickname { font-weight: 500; }
	.card-digits { color: var(--color-text-secondary); font-family: monospace; }

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: var(--color-primary);
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.9rem;
		cursor: pointer;
		min-height: 2.75rem;
	}
	.btn-primary:hover { background: var(--color-primary-hover); }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }

	.btn-small {
		padding: 0.4rem 0.75rem;
		font-size: 0.85rem;
		border-radius: 0.3rem;
		cursor: pointer;
		min-height: 2.75rem;
	}
	.btn-secondary { background: var(--color-bg-secondary); color: var(--color-text-label); border: 1px solid var(--color-border-input); }
	.btn-secondary:hover { background: var(--color-bg-secondary-hover); }
	.btn-danger { background: var(--color-danger-bg); color: var(--color-danger-text); border: 1px solid var(--color-danger-border); }
	.btn-danger:hover { background: var(--color-danger-hover); }

	.empty { color: var(--color-text-secondary); font-style: italic; }

	@media (max-width: 640px) {
		.card-form {
			flex-direction: column;
			align-items: stretch;
		}

		.card-form .field {
			width: 100%;
		}

		.card-form input {
			width: 100%;
		}

		.card-item {
			flex-direction: column;
			align-items: flex-start;
		}

		.card-info {
			width: 100%;
		}

		.card-actions {
			width: 100%;
		}

		.card-actions .btn-small {
			flex: 1;
			justify-content: center;
		}
	}
</style>
