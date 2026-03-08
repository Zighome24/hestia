import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ReceiptsPage from '../routes/receipts/+page.svelte';

vi.mock('$lib/api', () => ({
	get: vi.fn().mockResolvedValue([])
}));

describe('Receipt list page', () => {
	beforeEach(() => { vi.restoreAllMocks(); });

	it('renders the page title', () => {
		render(ReceiptsPage);
		expect(screen.getByRole('heading', { name: 'Receipts', level: 1 })).toBeInTheDocument();
	});

	it('renders the add receipt button', () => {
		render(ReceiptsPage);
		expect(screen.getByRole('link', { name: 'Add Receipt' })).toHaveAttribute('href', '/receipts/new');
	});
});
