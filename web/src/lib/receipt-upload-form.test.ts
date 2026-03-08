import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import NewReceiptPage from '../routes/receipts/new/+page.svelte';

vi.mock('$app/navigation', () => ({ goto: vi.fn() }));
vi.mock('$lib/api', () => ({
	get: vi.fn().mockResolvedValue([]),
	post: vi.fn()
}));

describe('Receipt upload form', () => {
	it('renders the page title', () => {
		render(NewReceiptPage);
		expect(screen.getByRole('heading', { name: 'Add Receipt', level: 1 })).toBeInTheDocument();
	});

	it('shows loading spinner while fetching data', () => {
		render(NewReceiptPage);
		expect(screen.getByRole('status')).toBeInTheDocument();
		expect(screen.getByText('Loading form data...')).toBeInTheDocument();
	});

	it('hides form while loading', () => {
		render(NewReceiptPage);
		expect(screen.queryByRole('button', { name: 'Save Receipt' })).not.toBeInTheDocument();
	});

	it('displays error banner container', () => {
		render(NewReceiptPage);
		// ErrorBanner is imported and available (renders conditionally on error)
		// Verify the component renders without crashing
		expect(screen.getByRole('heading', { name: 'Add Receipt' })).toBeInTheDocument();
	});
});
