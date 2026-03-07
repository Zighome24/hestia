import { writable } from 'svelte/store';
import { get, post } from './api';

export interface User {
	id: string;
	username: string;
	display_name: string;
	created_at: string;
}

export const currentUser = writable<User | null>(null);

export async function checkAuth(): Promise<User | null> {
	try {
		const user = await get<User>('/auth/me');
		currentUser.set(user);
		return user;
	} catch {
		currentUser.set(null);
		return null;
	}
}

export async function login(username: string, password: string): Promise<User> {
	const user = await post<User>('/auth/login', { username, password });
	currentUser.set(user);
	return user;
}

export async function register(
	username: string,
	displayName: string,
	password: string
): Promise<User> {
	const user = await post<User>('/auth/register', {
		username,
		display_name: displayName,
		password
	});
	currentUser.set(user);
	return user;
}

export async function logout(): Promise<void> {
	await post<void>('/auth/logout', {});
	currentUser.set(null);
}
