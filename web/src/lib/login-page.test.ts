import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import LoginPage from '../routes/login/+page.svelte';

// Mock $app/navigation
vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

// Mock $lib/auth
vi.mock('$lib/auth', () => ({
	login: vi.fn(),
	currentUser: {
		subscribe: vi.fn((cb) => {
			cb(null);
			return () => {};
		})
	}
}));

describe('Login page', () => {
	beforeEach(() => {
		vi.restoreAllMocks();
	});

	it('renders the login form with required fields', () => {
		render(LoginPage);

		expect(screen.getByLabelText('Username')).toBeInTheDocument();
		expect(screen.getByLabelText('Password')).toBeInTheDocument();
		expect(screen.getByRole('button', { name: 'Sign In' })).toBeInTheDocument();
	});

	it('renders username input with correct attributes', () => {
		render(LoginPage);

		const usernameInput = screen.getByLabelText('Username');
		expect(usernameInput).toHaveAttribute('type', 'text');
		expect(usernameInput).toHaveAttribute('autocomplete', 'username');
		expect(usernameInput).toBeRequired();
	});

	it('renders password input with correct attributes', () => {
		render(LoginPage);

		const passwordInput = screen.getByLabelText('Password');
		expect(passwordInput).toHaveAttribute('type', 'password');
		expect(passwordInput).toHaveAttribute('autocomplete', 'current-password');
		expect(passwordInput).toBeRequired();
	});

	it('has a link to registration page', () => {
		render(LoginPage);

		const registerLink = screen.getByRole('link', { name: 'Register' });
		expect(registerLink).toHaveAttribute('href', '/register');
	});
});
