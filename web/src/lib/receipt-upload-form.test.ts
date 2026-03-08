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

	it('renders amount input', () => {
		render(NewReceiptPage);
		expect(screen.getByLabelText('Amount ($)')).toBeInTheDocument();
	});

	it('renders date input', () => {
		render(NewReceiptPage);
		expect(screen.getByLabelText('Date')).toBeInTheDocument();
	});

	it('renders photo input with accept attribute', () => {
		render(NewReceiptPage);
		const photoInput = screen.getByLabelText('Photo');
		expect(photoInput).toHaveAttribute('accept', 'image/jpeg,image/png');
	});

	it('renders submit button', () => {
		render(NewReceiptPage);
		expect(screen.getByRole('button', { name: 'Save Receipt' })).toBeInTheDocument();
	});
});
