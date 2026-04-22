import { ThemeProvider as NextThemesProvider } from 'next-themes';
import { NextUIProvider } from '@nextui-org/react';
import ReactDOM from 'react-dom/client';
import React from 'react';

import { initStore } from './utils/store';
import { initEnv } from './utils/env';
import App from './App';

if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    });
}

async function start() {
    try {
        await initStore();
    } catch (e) {
        console.error('initStore failed:', e);
    }
    try {
        await initEnv();
    } catch (e) {
        console.error('initEnv failed:', e);
    }
    const rootElement = document.getElementById('root');
    if (!rootElement) {
        console.error('Root element not found!');
        return;
    }
    const root = ReactDOM.createRoot(rootElement);
    root.render(
        <NextUIProvider>
            <NextThemesProvider attribute='class'>
                <App />
            </NextThemesProvider>
        </NextUIProvider>
    );
}

start();
