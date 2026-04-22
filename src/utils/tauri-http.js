/**
 * Tauri v2 HTTP wrapper using @tauri-apps/plugin-http
 * This avoids CORS issues in dev mode by using native HTTP client
 */

import { fetch as tauriFetch } from '@tauri-apps/plugin-http';

export const Body = {
    text(text) {
        return { type: 'Text', payload: text };
    },
    json(data) {
        return { type: 'Json', payload: JSON.stringify(data) };
    },
    bytes(bytes) {
        return { type: 'Bytes', payload: bytes };
    },
    form(fields) {
        return { type: 'Form', payload: fields };
    },
};

export async function fetch(url, options = {}) {
    // Handle query parameters
    if (options.query) {
        const urlObj = new URL(url);
        Object.entries(options.query).forEach(([key, value]) => {
            urlObj.searchParams.append(key, value);
        });
        url = urlObj.toString();
    }

    // Handle v1-style body objects: { type: "Form", payload: {...} }
    let body = options.body;
    if (body && typeof body === 'object' && 'type' in body && 'payload' in body) {
        // Already in v1 format, convert to native fetch
        if (body.type === 'Form') {
            const formData = new FormData();
            for (const [key, value] of Object.entries(body.payload)) {
                if (value && typeof value === 'object' && 'file' in value) {
                    const blob = new Blob([value.file], { type: value.mime || 'application/octet-stream' });
                    formData.append(key, blob, value.fileName || 'file');
                } else {
                    formData.append(key, value);
                }
            }
            body = formData;
        } else if (body.type === 'Json') {
            body = JSON.stringify(body.payload);
        } else if (body.type === 'Text') {
            body = body.payload;
        }
    }

    // Build fetch options
    const fetchOptions = {
        method: options.method || 'GET',
        headers: options.headers || options.header || {},
    };
    if (fetchOptions.method !== 'GET' && fetchOptions.method !== 'HEAD' && body !== undefined) {
        fetchOptions.body = body;
    }

    // Call Tauri native fetch (no CORS issues)
    const response = await tauriFetch(url, fetchOptions);

    // Parse response body
    let data;
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
        try {
            data = await response.json();
        } catch {
            data = await response.text();
        }
    } else {
        data = await response.text();
    }

    return {
        ok: response.ok,
        status: response.status,
        statusText: response.statusText,
        data,
    };
}
