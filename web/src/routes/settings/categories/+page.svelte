<script lang="ts">
	import { onMount } from 'svelte';
	import { get, post, put, del } from '$lib/api';
	import Spinner from '$lib/components/Spinner.svelte';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';

	interface Category { id: string; name: string; color: string; created_at: string; }

	let categories: Category[] = [];
	let loading = true;
	let error = '';

	let newName = '';
	let newColor = '#6b7280';
	let addError = '';
	let adding = false;

	let editingId: string | null = null;
	let editName = '';
	let editColor = '';
	let editError = '';

	onMount(async () => {
		await loadCategories();
	});

	async function loadCategories() {
		loading = true;
		try {
			categories = await get<Category[]>('/categories');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load';
		} finally {
			loading = false;
		}
	}

	async function handleAdd(e: SubmitEvent) {
		e.preventDefault();
		addError = '';
		if (!newName.trim()) { addError = 'Name is required'; return; }
		adding = true;
		try {
			const cat = await post<Category>('/categories', { name: newName.trim(), color: newColor });
			categories = [...categories, cat].sort((a, b) => a.name.localeCompare(b.name));
			newName = '';
			newColor = '#6b7280';
		} catch (err) {
			addError = err instanceof Error ? err.message : 'Failed to add';
		} finally {
			adding = false;
		}
	}

	function startEdit(cat: Category) { editingId = cat.id; editName = cat.name; editColor = cat.color; editError = ''; }
	function cancelEdit() { editingId = null; editError = ''; }

	async function handleEdit(e: SubmitEvent) {
		e.preventDefault();
		editError = '';
		if (!editName.trim()) { editError = 'Name is required'; return; }
		try {
			const updated = await put<Category>(`/categories/${editingId}`, { name: editName.trim(), color: editColor });
			categories = categories.map(c => c.id === editingId ? updated : c);
			editingId = null;
		} catch (err) {
			editError = err instanceof Error ? err.message : 'Failed to update';
		}
	}

	async function handleDelete(id: string) {
		if (!confirm('Delete this category?')) return;
		try {
			await del(`/categories/${id}`);
			categories = categories.filter(c => c.id !== id);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete';
		}
	}
</script>

<h1>Categories</h1>

{#if error}
	<ErrorBanner message={error} on:dismiss={() => (error = '')} />
{/if}

<section class="add-section">
	<h2>Add Category</h2>
	{#if addError}
		<ErrorBanner message={addError} on:dismiss={() => (addError = '')} />
	{/if}
	<form class="cat-form" on:submit={handleAdd}>
		<div class="field">
			<label for="new-name">Name</label>
			<input id="new-name" type="text" bind:value={newName} required />
		</div>
		<div class="field">
			<label for="new-color">Color</label>
			<input id="new-color" type="color" bind:value={newColor} />
		</div>
		<button type="submit" class="btn-primary" disabled={adding}>{adding ? 'Adding...' : 'Add'}</button>
	</form>
</section>

<section>
	<h2>Your Categories</h2>
	{#if loading}
		<Spinner message="Loading categories..." />
	{:else if categories.length === 0}
		<p class="empty">No categories yet.</p>
	{:else}
		<ul class="cat-list">
			{#each categories as cat (cat.id)}
				<li class="cat-item">
					{#if editingId === cat.id}
						{#if editError}
							<ErrorBanner message={editError} on:dismiss={() => (editError = '')} />
						{/if}
						<form class="cat-form" on:submit={handleEdit}>
							<input type="text" bind:value={editName} required />
							<input type="color" bind:value={editColor} />
							<button type="submit" class="btn-small btn-primary">Save</button>
							<button type="button" class="btn-small btn-secondary" on:click={cancelEdit}>Cancel</button>
						</form>
					{:else}
						<div class="cat-info">
							<span class="cat-swatch" style="background: {cat.color}"></span>
							<span class="cat-name">{cat.name}</span>
						</div>
						<div class="cat-actions">
							<button class="btn-small btn-secondary" on:click={() => startEdit(cat)}>Edit</button>
							<button class="btn-small btn-danger" on:click={() => handleDelete(cat.id)}>Delete</button>
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
	.cat-form { display: flex; gap: 0.75rem; align-items: flex-end; flex-wrap: wrap; }
	.field { display: flex; flex-direction: column; gap: 0.25rem; }
	label { font-size: 0.8rem; font-weight: 500; color: var(--color-text-label); }
	input[type="text"] { padding: 0.4rem 0.6rem; border: 1px solid var(--color-border-input); border-radius: 0.375rem; font-size: 0.95rem; min-height: 2.5rem; background: var(--color-bg-card); color: var(--color-text); }
	input[type="text"]:focus { outline: none; border-color: var(--color-primary); box-shadow: 0 0 0 2px var(--color-focus-shadow); }
	input[type="color"] { width: 3rem; min-height: 2.5rem; border: 1px solid var(--color-border-input); border-radius: 0.375rem; cursor: pointer; padding: 2px; }
	.cat-list { list-style: none; display: flex; flex-direction: column; gap: 0.5rem; }
	.cat-item { display: flex; align-items: center; justify-content: space-between; padding: 0.6rem 1rem; background: var(--color-bg-card); border: 1px solid var(--color-border); border-radius: 0.5rem; flex-wrap: wrap; gap: 0.5rem; }
	.cat-info { display: flex; align-items: center; gap: 0.5rem; }
	.cat-swatch { width: 1rem; height: 1rem; border-radius: 50%; border: 1px solid var(--color-border-input); }
	.cat-name { font-weight: 500; }
	.cat-actions { display: flex; gap: 0.5rem; }
	.btn-primary { padding: 0.4rem 1rem; background: var(--color-primary); color: white; border: none; border-radius: 0.375rem; font-size: 0.9rem; cursor: pointer; min-height: 2.75rem; }
	.btn-primary:hover { background: var(--color-primary-hover); }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
	.btn-small { padding: 0.3rem 0.7rem; font-size: 0.85rem; border-radius: 0.3rem; cursor: pointer; min-height: 2.75rem; }
	.btn-secondary { background: var(--color-bg-secondary); color: var(--color-text-label); border: 1px solid var(--color-border-input); }
	.btn-secondary:hover { background: var(--color-bg-secondary-hover); }
	.btn-danger { background: var(--color-danger-bg); color: var(--color-danger-text); border: 1px solid var(--color-danger-border); }
	.btn-danger:hover { background: var(--color-danger-hover); }
	.empty { color: var(--color-text-secondary); font-style: italic; }

	@media (max-width: 640px) {
		.cat-form {
			flex-direction: column;
			align-items: stretch;
		}

		.cat-form .field {
			width: 100%;
		}

		.cat-form input[type="text"] {
			width: 100%;
		}

		.cat-item {
			flex-direction: column;
			align-items: flex-start;
		}

		.cat-info {
			width: 100%;
		}

		.cat-actions {
			width: 100%;
		}

		.cat-actions .btn-small {
			flex: 1;
			justify-content: center;
		}
	}
</style>
