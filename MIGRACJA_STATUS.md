# Pot-F Desktop - Polski fork z migracją na Tauri v2

## 📋 Status Migracji i Napraw

### ✅ Ukończone Zadania

1. **Migracja na Tauri v2** - ZAKOŃCZONA
   - Zaktualizowano `Cargo.toml` do wersji 2.x wszystkich pluginów
   - Zaktualizowano `package.json` do `@tauri-apps/api@^2` i pluginów v2
   - Przepisano kod Rust na nowe API Tauri v2:
     - `tauri::WebviewWindow` zamiast `tauri::Window`
     - `WebviewWindowBuilder` zamiast `WindowBuilder`
     - Nowe API `global_shortcut` z `tauri_plugin_global_shortcut`
     - Nowe API clipboard z `tauri_plugin_clipboard_manager`
   - Dodano plik capabilities dla Tauri v2 security model

2. **Polska Lokalizacja** - ZAKOŃCZONA
   - Plik `src/i18n/locales/pl_PL.json` jako domyślny język
   - `app_language` ustawiony na `"pl"` w konfiguracji
   - Wszystkie komunikaty UI przetłumaczone na polski

3. **Naprawa Capabilities (Tauri v2)** - ZAKOŃCZONA
   - Rozbudowany plik `src-tauri/capabilities/default.json`
   - Dodane wszystkie wymagane permisje dla:
     - shell, fs, dialog, clipboard-manager
     - notification, global-shortcut, process, os
     - updater, autostart, store, log, sql, http
     - core:window, core:webview, core:image

4. **Wsparcie dla Kiri Linux (Arch fork)** - DODANE
   - Stworzono `arch/PKGBUILD` z zależnościami dla Arch/Kiri
   - Zależności: `webkit2gtk-4.1`, `gtk3`, `libayatana-appindicator`, `xclip`, `dbus`
   - Opcjonalne pakiety językowe dla OCR (PL, DE, FR, RU, UK)
   - Poprawiono build() function z pustą komendą `:`

5. **Rozbudowane Zależności Linux** - DODANE
   - `tauri.conf.json` z pełnymi zależnościami dla:
     - DEB (Debian/Ubuntu)
     - RPM (Fedora/Kiri/RHEL)
     - AppImage (uniwersalny)

6. **Naprawa selection::get_text dla Linux** - NAPRAWIONE
   - Usunięto zależność od `selection` crate (nie działa poprawnie na Linux/Wayland)
   - Zaimplementowano platform-specific getter:
     - Linux: używa `xclip` i `xdotool` do pobrania zaznaczonego tekstu
     - Windows/macOS: używa oryginalnego `selection::get_text()`
   - Dodano fallback mechanizmy dla różnych środowisk graficznych

### 🔧 Problemy z Kompilacją - NAPRAWIONE

**Główne przyczyny błędów:**
1. ❌ Brakujące permisje w `capabilities/default.json`
2. ❌ Niekompatybilne API Tauri v1/v2
3. ❌ Brakujące zależności systemowe
4. ❌ `selection` crate nie działa na Linux bez X11

**Rozwiązania:**
1. ✅ Pełna lista permisji w capabilities
2. ✅ Kod zaktualizowany do API Tauri v2
3. ✅ Dokumentacja zależności dla Fedora/Kiri/Arch + xclip
4. ✅ Platform-specific code dla get_selected_text()

---

## 🚀 Budowanie na Kiri Linux

### Wymagania Systemowe

```bash
# Dla Kiri/Arch Linux
sudo pacman -S webkit2gtk-4.1 gtk3 libayatana-appindicator librsvg \
    tesseract tesseract-data-eng tesseract-data-pol \
    xdotool libxcb libxrandr dbus xclip \
    nodejs>=18.0.0 pnpm rust>=1.80.0
```

### Budowanie z AUR

```bash
# Sklonuj repozytorium
cd /tmp
git clone https://github.com/frs777/pot-f-desktop.git
cd pot-f-desktop/arch

# Zbuduj paczkę
makepkg -si

# Lub użyj yay/paru
yay -S pot-f-desktop
```

### Budowanie Ręczne

```bash
# Instalacja zależności
pnpm install

# Build aplikacji
pnpm tauri build

# Wynikowe paczki będą w:
# - src-tauri/target/release/bundle/deb/
# - src-tauri/target/release/bundle/rpm/
# - src-tauri/target/release/bundle/appimage/
```

---

## 📦 Struktura Projektu

```
/workspace/
├── src/                      # Frontend React + TypeScript
│   ├── components/           # Komponenty UI
│   ├── i18n/                 # Lokalizacje (pl_PL domyślnie)
│   └── services/             # Usługi tłumaczenia/OCR
├── src-tauri/                # Backend Rust + Tauri v2
│   ├── src/                  # Kod Rust (zmigrowany na v2)
│   │   ├── window.rs         # NAPRAWIONO: get_selected_text()
│   │   └── ...
│   ├── capabilities/         # Security permissions (v2)
│   │   └── default.json      # ROZBUDOWANE PERMISJE
│   ├── icons/                # Ikony aplikacji
│   └── tauri.conf.json       # Konfiguracja Tauri
├── arch/                     # Paczki dla Arch/Kiri
│   └── PKGBUILD              # ZAKTUALIZOWANY - webkit2gtk-4.1, xclip, dbus
└── asset/                    # Zasoby
    └── com.pot-f-desktop.desktop  # Desktop entry
```

---

## 🔍 Szczegóły Techniczne Migracji

### Zmiany w Kodzie Rust

**Tauri v1 → v2:**

```rust
// v1
use tauri::{Window, WindowBuilder};

// v2
use tauri::webview::{WebviewWindow, WebviewWindowBuilder};
```

**Global Shortcut:**

```rust
// v1
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

// v2 - to samo, ale z nowym API eventów
app_handle.global_shortcut().on_shortcut(shortcut, |app, sc, event| {
    if matches!(event.state(), ShortcutState::Pressed) {
        // handler
    }
});
```

**Clipboard:**

```rust
// v1
use tauri_plugin_clipboard_manager::ClipboardExt;

// v2 - API bez zmian, ale wymaga permisji
app.clipboard().read_text()?;
```

**Selection Text (Linux vs inne):**

```rust
// v1 - uniwersalne
use selection::get_text;
let text = get_text();

// v2 - platform-specific
#[cfg(target_os = "linux")]
fn get_selected_text() -> String {
    // użycie xclip/xdotool
}

#[cfg(not(target_os = "linux"))]
fn get_selected_text() -> String {
    use selection::get_text;
    get_text()
}
```

### Nowe Permisje (capabilities)

Plik `src-tauri/capabilities/default.json` zawiera teraz:
- 150+ indywidualnych permisji
- Podział na moduły (shell, fs, dialog, etc.)
- Jawne zezwolenia na operacje (allow-*)

---

## ✅ Lista Kontrolna

- [x] Migracja Cargo.toml na Tauri v2
- [x] Migracja package.json na @tauri-apps v2
- [x] Aktualizacja kodu Rust (window, webview, shortcuts)
- [x] Utworzenie capabilities/default.json z pełnymi permisjami
- [x] Polska lokalizacja (pl_PL.json)
- [x] PKGBUILD dla Arch/Kiri Linux
- [x] Desktop entry file
- [x] Zależności dla RPM/DEB/AppImage
- [x] Ikony w rozmiarach 32-512px
- [x] Naprawa selection_text dla Linux (xclip/xdotool)
- [x] Dodanie xclip i dbus do zależności PKGBUILD
- [x] Zmiana webkit2gtk-5 na webkit2gtk-4.1

---

## 🐛 Znane Problemy

1. **WebKit2GTK wersja**: Używamy `webkit2gtk-4.1` (kompatybilny z większością dystrybucji)
2. **AppIndicator**: Na niektórych dystrybucjach potrzebny `libappindicator-gtk3`
3. **Tesseract OCR**: Wymaga osobnej instalacji danych językowych
4. **Wayland**: Funkcja wyboru tekstu może wymagać XWayland dla xclip

---

## 📞 Wsparcie

W przypadku problemów:
1. Sprawdź zależności systemowe
2. Upewnij się że Rust >= 1.80.0
3. Wyczyść cache: `cargo clean && pnpm clean`
4. Sprawdź logi: `~/.config/com.pot-f-desktop.desktop/logs/`
5. Dla Wayland: zainstaluj `xwayland` i uruchom aplikację przez XWayland
