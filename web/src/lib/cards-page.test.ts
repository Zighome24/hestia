import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import CardsPage from '../routes/settings/cards/+page.svelte';

// Mock the API module
vi.mock('$lib/api', () => ({
	get: vi.fn().mockResolvedValue([]),
	post: vi.fn(),
	put: vi.fn(),
	del: vi.fn()
}));

describe('Cards management page', () => {
	beforeEach(() => {
		vi.restoreAllMocks();
	});

	it('renders the page title', () => {
		render(CardsPage);
		expect(screen.getByRole('heading', { name: 'Credit Cards', level: 1 })).toBeInTheDocument();
	});

	it('renders the add card form with required fields', () => {
		render(CardsPage);
		expect(screen.getByLabelText('Nickname')).toBeInTheDocument();
		expect(screen.getByLabelText('Last 4 Digits')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Add Card' })).toBeInTheDocument();
	});

	it('nickname input has required attribute', () => {
		render(CardsPage);
		expect(screen.getByLabelText('Nickname')).toBeRequired();
	});

	it('last four input has maxlength and pattern', () => {
		render(CardsPage);
		const input = screen.getByLabelText('Last 4 Digits');
		expect(input).toHaveAttribute('maxlength', '4');
		expect(input).toHaveAttribute('pattern', '\\d{4}'); // DOM attr: \d{4}
	});
});
