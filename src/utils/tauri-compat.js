// Compatibility wrappers for Tauri v1 -> v2 migration
import { getCurrentWindow } from '@tauri-apps/api/window';

// appWindow wrapper - returns current window instance
export const appWindow = {
    get label() {
        return getCurrentWindow().label;
    },
    close: () => getCurrentWindow().close(),
    show: () => getCurrentWindow().show(),
    hide: () => getCurrentWindow().hide(),
    setFocus: () => getCurrentWindow().setFocus(),
    setResizable: (resizable) => getCurrentWindow().setResizable(resizable),
    setAlwaysOnTop: (alwaysOnTop) => getCurrentWindow().setAlwaysOnTop(alwaysOnTop),
    outerPosition: () => getCurrentWindow().outerPosition(),
    outerSize: () => getCurrentWindow().outerSize(),
    innerPosition: () => getCurrentWindow().innerPosition(),
    innerSize: () => getCurrentWindow().innerSize(),
};

// currentMonitor wrapper
export { currentMonitor } from '@tauri-apps/api/window';
