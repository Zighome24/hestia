import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get, post, put, del } from './api';

describe('API client', () => {
	beforeEach(() => {
		vi.restoreAllMocks();
	});

	it('get() calls fetch with GET method and correct URL', async () => {
		const mockResponse = { data: 'test' };
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(mockResponse)
			})
		);

		const result = await get('/health');

		expect(fetch).toHaveBeenCalledWith('/api/health', {
			method: 'GET',
			headers: { 'Content-Type': 'application/json' }
		});
		expect(result).toEqual(mockResponse);
	});

	it('post() sends JSON body with POST method', async () => {
		const body = { username: 'test', password: 'secret' };
		const mockResponse = { id: 1 };
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(mockResponse)
			})
		);

		const result = await post('/auth/login', body);

		expect(fetch).toHaveBeenCalledWith('/api/auth/login', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(body)
		});
		expect(result).toEqual(mockResponse);
	});

	it('put() sends JSON body with PUT method', async () => {
		const body = { nickname: 'Updated Card' };
		const mockResponse = { id: 1, nickname: 'Updated Card' };
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(mockResponse)
			})
		);

		const result = await put('/cards/1', body);

		expect(fetch).toHaveBeenCalledWith('/api/cards/1', {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(body)
		});
		expect(result).toEqual(mockResponse);
	});

	it('del() calls fetch with DELETE method', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: true,
				status: 204,
				json: () => Promise.resolve(undefined)
			})
		);

		await del('/cards/1');

		expect(fetch).toHaveBeenCalledWith('/api/cards/1', {
			method: 'DELETE',
			headers: { 'Content-Type': 'application/json' }
		});
	});

	it('throws on non-2xx response with error message', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: false,
				status: 401,
				json: () => Promise.resolve({ error: 'unauthorized' })
			})
		);

		await expect(get('/protected')).rejects.toThrow('API error 401: unauthorized');
	});

	it('throws on non-2xx response with text fallback', async () => {
		vi.stubGlobal(
			'fetch',
			vi.fn().mockResolvedValue({
				ok: false,
				status: 500,
				json: () => Promise.reject(new Error('not json')),
				text: () => Promise.resolve('Internal Server Error')
			})
		);

		await expect(get('/broken')).rejects.toThrow('API error 500: Internal Server Error');
	});
});
