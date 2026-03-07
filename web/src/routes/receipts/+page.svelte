<script lang="ts">
</script>

<div class="header">
	<h1>Receipts</h1>
	<a href="/receipts/new" class="btn-primary">Add Receipt</a>
</div>

<p class="empty">No receipts yet.</p>

<style>
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.btn-primary {
		padding: 0.5rem 1rem;
		background: #2563eb;
		color: white;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.btn-primary:hover {
		background: #1d4ed8;
		text-decoration: none;
	}

	.empty {
		color: #6b7280;
		text-align: center;
		padding: 2rem 0;
	}
</style>
