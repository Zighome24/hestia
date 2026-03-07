import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import RegisterPage from '../routes/register/+page.svelte';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/auth', () => ({
	register: vi.fn(),
	currentUser: {
		subscribe: vi.fn((cb) => {
			cb(null);
			return () => {};
		})
	}
}));

describe('Register page', () => {
	beforeEach(() => {
		vi.restoreAllMocks();
	});

	it('renders the registration form with all required fields', () => {
		render(RegisterPage);

		expect(screen.getByLabelText('Username')).toBeInTheDocument();
		expect(screen.getByLabelText('Display Name')).toBeInTheDocument();
		expect(screen.getByLabelText('Password')).toBeInTheDocument();
		expect(screen.getByLabelText('Confirm Password')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Create Account' })).toBeInTheDocument();
	});

	it('has a link to login page', () => {
		render(RegisterPage);

		const loginLink = screen.getByRole('link', { name: 'Sign in' });
		expect(loginLink).toHaveAttribute('href', '/login');
	});
});
