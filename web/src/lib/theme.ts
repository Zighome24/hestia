import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

const STORAGE_KEY = 'hestia-theme';

function getInitialTheme(): Theme {
	if (browser) {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored === 'light' || stored === 'dark') return stored;
	}
	return 'light';
}

export const theme = writable<Theme>(getInitialTheme());

export function toggleTheme() {
	theme.update((current) => {
		const next = current === 'light' ? 'dark' : 'light';
		if (browser) {
			localStorage.setItem(STORAGE_KEY, next);
			document.documentElement.setAttribute('data-theme', next);
		}
		return next;
	});
}

// Initialize the attribute on first load
if (browser) {
	const stored = localStorage.getItem(STORAGE_KEY);
	if (stored === 'dark') {
		document.documentElement.setAttribute('data-theme', 'dark');
	}
}
