// Configure via PUBLIC_API_URL env var at build time, defaults to /api
const API_BASE = '/api';

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
	const url = `${API_BASE}${path}`;
	const init: RequestInit = {
		method,
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include'
	};

	if (body !== undefined) {
		init.body = JSON.stringify(body);
	}

	const response = await fetch(url, init);

	if (!response.ok) {
		let message: string;
		try {
			const data = await response.json();
			message = data.message ?? data.error ?? JSON.stringify(data);
		} catch {
			message = await response.text();
		}
		throw new Error(`API error ${response.status}: ${message}`);
	}

	const text = await response.text();
	if (!text) {
		return undefined as T;
	}

	return JSON.parse(text) as T;
}

export function get<T>(path: string): Promise<T> {
	return request<T>('GET', path);
}

export function post<T>(path: string, body: unknown): Promise<T> {
	return request<T>('POST', path, body);
}

export function put<T>(path: string, body: unknown): Promise<T> {
	return request<T>('PUT', path, body);
}

export function del(path: string): Promise<void> {
	return request<void>('DELETE', path);
}
