import { fetch } from '../../../utils/tauri-http.js';

export const Language = {
    en: 'en',
    zh: 'zh',
    ja: 'ja',
    ko: 'ko',
    fr: 'fr',
    de: 'de',
    es: 'es',
    ru: 'ru',
    pl: 'pl',
    it: 'it',
    pt: 'pt',
    ar: 'ar',
    tr: 'tr',
    nl: 'nl',
    id: 'id',
    th: 'th',
    vi: 'vi',
    auto: 'auto',
};

export async function translate(text, from, to, options = {}) {
    const { config } = options;

    let { requestPath = 'lingva.pot-app.com' } = config || {};

    if (requestPath.length === 0) {
        requestPath = 'lingva.pot-app.com';
    }

    if (!requestPath.startsWith('http')) {
        requestPath = 'https://' + requestPath;
    }

    let plain_text = text.replaceAll('/', '@@');
    let encode_text = encodeURIComponent(plain_text);
    const res = await fetch(`${requestPath}/api/v1/${from}/${to}/${encode_text}`, {
        method: 'GET',
    });

    if (res.ok) {
        let result = res.data;
        const { translation } = result;
        if (translation) {
            return translation.replaceAll('@@', '/');
        } else {
            throw JSON.stringify(result.trim());
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}

export * from './Config';
export * from './info';
