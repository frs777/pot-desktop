# Pot-F Desktop - Polski fork z migracją na Tauri v2

## 📋 Status Migracji i Napraw

**Ostatnia aktualizacja:** 2026-08-06

### Aktualny stan weryfikacji

- Frontend (`pnpm build`) kompiluje się poprawnie.
- Backend Rust/Tauri (`pnpm tauri build --no-bundle`) kompiluje się poprawnie.
- Binarka powstaje w `src-tauri/target/release/pot-f`.
- Pełny `pnpm tauri build` tworzy pakiety DEB i RPM, ale etap AppImage kończy się błędem narzędzia `linuxdeploy`. AppImage nie jest obecnie wymagany.
- Utworzono paczkę testową Arch `arch/pot-f-desktop-0.1.0-9-x86_64.pkg.tar.zst`; `namcap` nie zgłasza błędów, jedynie ostrzeżenia o zależnościach funkcjonalnych. Paczkę `0.1.0-3` przeniesiono do kosza na życzenie użytkownika; pozostałe wersje testowe zachowano.
- Ustawienia są widoczne, natomiast okno tłumaczenia w `0.1.0-2` pozostawało puste. W `0.1.0-3` rama okna tłumaczenia renderuje się zawsze. W `0.1.0-4` przywrócono uprawnienia Tauri do zamykania, minimalizacji, maksymalizacji, przeciągania i przypinania okien; działanie kontrolek potwierdzono.
- Próba rozwiązania problemu przez zmianę wspólnego `src-tauri/src/window.rs` została wycofana. Właściwe źródło problemu znaleziono w `src-tauri/src/main.rs`: obejście wyłączające kompozytor WebKit i wymuszające renderowanie programowe było stosowane na każdym Linuksie. Od `0.1.0-2` jest stosowane tylko poza sesją Wayland.
- Kod `pot-f-desktop` zawiera znacznie więcej usług wbudowanych (m.in. Alibaba, Baidu, Bing, Caiyun, Cambridge, DeepL, Google, OpenAI, Gemini, Tencent, Volcengine, Youdao, OCR i TTS). Siedem usług było jedynie wcześniej aktywnych w konfiguracji użytkownika. Usługi wymagające kluczy API można dodać później z okna „Dodaj usługę”. Zewnętrzne pluginy społecznościowe pozostają przeznaczone do osobnej paczki AUR. W `0.1.0-5` naprawiono podwójne kodowanie JSON w wrapperze HTTP, a w `0.1.0-8` dodano pełne zakresy ścieżek dla znanych domen wszystkich usług wbudowanych.
- W `0.1.0-6` poprawiono synchronizację lokalnego tekstu z atomem Jotai: przekazywana jest jawnie bieżąca wartość, co usuwa możliwość uruchomienia tłumaczenia dla poprzedniego/pustego tekstu. Dodano trwałe logi `[translation-debug]` bez zapisywania treści użytkownika. `src-tauri/tauri.linux.conf.json` pozostaje pusty.
- W `0.1.0-7` naprawiono puste strony Usługi i Historia w ustawieniach, dodano uprawnienie zapisu SQL, dostęp Aliyun, walidację URL WebDAV, otwieranie katalogów logów/konfiguracji, przełącznik schowka w trayu oraz globalne logowanie wyjątków frontendu.
- W `0.1.0-8` poprawiono zakresy HTTP Tauri 2: domeny wszystkich usług wbudowanych obejmują teraz ich pełne ścieżki API. Dodano też dostęp rekurencyjny do katalogu `$APPCONFIG/plugins`, którego brak powodował błąd `forbidden path` przy ładowaniu wtyczek. Dowolne domeny wtyczek społecznościowych nie zostały globalnie dopuszczone ze względów bezpieczeństwa.
- W `0.1.0-9` naprawiono panic `Nie można pobrać monitora` przy tworzeniu ukrytego okna tłumaczenia na Waylandzie, dodano uprawnienia `window|show` i `window|set_focus` oraz wyłączono aktualizator niezgodnego upstream Pot `4.0.0`. Test binarki potwierdził otwarcie okna i tłumaczenie Google. Numer `0.1.0-9` jest widoczny w stronie O aplikacji.
- Repozytorium źródłowe: `https://github.com/frs777/pot-desktop.git`, bieżący commit: `ea3d1c4`.
- Tag `v0.1.0` nie istnieje jeszcze w zdalnym repozytorium, dlatego źródłowy PKGBUILD do publikacji w AUR wymaga jeszcze poprawnego URL/tagu i sum kontrolnych.

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

### Pakiet Arch / plan publikacji w AUR

Lokalna paczka testowa jest budowana z bieżącego drzewa roboczego. Publikację w AUR należy wykonać dopiero po usunięciu problemu przezroczystego okna na Waylandzie, wypchnięciu zmian i utworzeniu tagu wydania.

```bash
# Repozytorium źródłowe
git clone https://github.com/frs777/pot-desktop.git
cd pot-desktop/arch

# Po przygotowaniu poprawnego źródłowego PKGBUILD
makepkg -si
```

Pakiet `pot-f-desktop` nie jest jeszcze gotowy do instalacji przez `yay` lub `paru`.

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
4. **Tłumaczenie**: w `0.1.0-6` poprawiono synchronizację tekstu, w `0.1.0-8` zakresy HTTP, a w `0.1.0-9` panic monitora i uprawnienia pokazania okna. Bezpośredni test `0.1.0-9` na KDE/Wayland potwierdził działanie Google; pozostaje test po instalacji paczki.
5. **Wayland / zaznaczony tekst**: funkcja wyboru tekstu może wymagać XWayland dla `xclip`/`xdotool`.

---

## 📞 Wsparcie

W przypadku problemów:
1. Sprawdź zależności systemowe
2. Upewnij się że Rust >= 1.80.0
3. Wyczyść cache: `cargo clean && pnpm clean`
4. Sprawdź logi: `~/.config/com.pot-f-desktop.desktop/logs/`
5. Dla problemów z odczytem zaznaczenia na Waylandzie: zainstaluj `xwayland` i sprawdź `xclip`/`xdotool`.
6. Dla niewidocznego okna na KDE/Wayland: nie modyfikuj wspólnego `window.rs`. Sprawdź warunkowe ustawianie zmiennych WebKit/GL w `src-tauri/src/main.rs`; paczka testowa z poprawką ma `pkgrel=2`.
