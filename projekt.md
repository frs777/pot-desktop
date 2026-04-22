# POT Desktop — Stan Projektu

> **Data:** 22 kwietnia 2026 — TŁUMACZENIE DZIAŁA ✅
> **Projekt:** pot-app/pot-desktop v3.0.7
> **Lokalizacja:** `/home/apawlowski/Oprogramowanie/TŁUMACZENIA/pot/pot-desktop`

---

## 1. Co jest gotowe

### ✅ Migracja Tauri 1.x → 2.x — UKOŃCZONA

- **Backend Rust**: Wszystkie 14 plików .rs zmigrowane do Tauri 2 API
- **Cargo.toml**: Tauri 2.x + wszystkie pluginy v2
- **tauri.conf.json**: Format v2 (`app:`, `bundle:`, `identifier:`, `build:`)
- **capabilities/default.json**: Uprawnienia v2 z wildcard HTTP scope
- **Frontend React**: Wszystkie importy zamienione na compat wrappery
- **cargo check**: ✅ 0 błędów
- **Wtyczki (35 sztuk)**: Spakowane w `plugins.tar.gz` (209MB)

### ✅ Tłumaczenie — DZIAŁA

- **DeepL** ✅ — działa poprawnie
- **Bing** ✅ — naprawiono (dodano obsługę `query` w tauri-http.js)
- **Google** ✅ — naprawiono (dodano obsługę `query` w tauri-http.js)
- **Yandex** ⚠️ — API v1 deprecated, zwraca 400 "Invalid parameter: lang"
- **Lingva** ❌ — serwer `lingva.pot-app.com` nie odpowiada (DNS/connection error)
- **ecdict** ❌ — słownik EN-ZH, nie obsługuje języka polskiego

### ✅ Tray Menu — Częściowo Działa

- **Ustawienia** ✅ — otwiera okno konfiguracji
- **Pokaż okno** ✅ — tworzy/pokazuje okno translacji
- **Schowek toggle** ⚠️ — event łapany, ale label nie aktualizuje się w menu
- **Wyjdź** ⚠️ — event łapany, `std::process::exit(0)` zamyka ale nie jest czyste

### ✅ Clipboard Monitoring

- Działa w tle — czyta schowek co 500ms
- Automatyczne tłumaczenie tekstu ze schowka

---

## 2. Zmiany Wprowadzone (22.04.2026) — Szczegółowa Lista

### A. HTTP / Sieć

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 1 | `src/utils/tauri-http.js` | Dodano obsługę parametrów `query` — konwersja obiektu `query` na URL search params | Bing/Google/Lingva używały `query: { from, to, api-version }` które były ignorowane. Bez tego URL nie miał parametrów i API zwracało błędy. |
| 2 | `src/services/translate/lingva/index.jsx` | Usunięto import `Body` (nieistniejący w Tauri v2), dodano konfigurowalny `requestPath` z options | `Body` nie jest eksportowane z plugin-http v2. Lingva używała hardcoded URL. |
| 3 | `src/services/translate/lingva/Config.jsx` | Przepisany — dodano pole `requestPath` jak w TTS Lingva, test połączenia przy zapisie | Umożliwia zmianę domyślnego serwera Lingva gdy `lingva.pot-app.com` nie działa |
| 4 | `src-tauri/capabilities/default.json` | Dodano wildcard URL scope: `"https://*"`, `"http://*"` do `http:default` | Bez tego plugin-http blokował wszystkie requesty z błędem "url not allowed on the configured scope" |

### B. Tray Menu

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 5 | `src-tauri/src/tray.rs` | Przepisany od podstaw: <br>• Użyto `TrayIconBuilder::with_id(TrayIconId::new(TRAY_ID))` <br>• `app.tray_by_id(TRAY_ID)` do sprawdzania czy tray istnieje <br>• `existing_tray.set_menu()` do aktualizacji bez tworzenia nowego <br>• Globalny `handle_menu_event` z match na ID <br>• `quit` jako zwykły `MenuItem` (nie `PredefinedMenuItem`) | Poprzednia wersja tworzyła NOWY tray przy każdej aktualizacji → podwajała ikony. Event handlers nie działały po aktualizacji menu. `PredefinedMenuItem::quit` nie wywoływał naszego handlera. |
| 6 | `src-tauri/src/main.rs` | Dodano `app_handle.on_menu_event(tray::handle_menu_event)` po build_tray | W Tauri v2 `on_menu_event` musi być zarejestrowany globalnie na AppHandle |
| 7 | `src-tauri/src/main.rs` | Dodano `toggle_clipboard_monitor` do `invoke_handler` | Aby frontend mógł wywołać toggle przez `invoke()` |

### C. Clipboard

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 8 | `src-tauri/src/clipboard.rs` | Dodano funkcję `toggle_clipboard_monitor()` — toggle stanu + `set("clipboard_monitor", ...)` + rebuild tray z nowym labelem | Brakowało mechanizmu przełączania schowka z poziomu UI/tray |

### D. Frontend — Event Handling

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 9 | `src/window/Translate/index.jsx` | Dodano `useEffect` z listenerami na `open-settings` i `toggle-clipboard-monitor` | Tray emituje te eventy ale frontend ich nie obsługiwał |
| 10 | `src/window/Translate/index.jsx` | Utrzymano `pluginList && (` conditional rendering | Bez tego interface nie renderował się gdy pluginList był null |

### E. Lokalizacja (i18n)

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 11 | `src/i18n/locales/pl_PL.json` | Dodano `"request_path": "Ścieżka żądania"` do `services.lingva` | Etykieta dla nowego pola konfiguracji Lingva |
| 12 | `src/i18n/locales/en_US.json` | Dodano `"request_path": "Request Path"` do `services.lingva` | Etykieta angielska |

### F. Pozostałe

| # | Plik | Zmiana | Dlaczego |
|---|------|--------|----------|
| 13 | `src/utils/tauri-http.js` | Eksport `Body` class z metodami `text()`, `json()`, `bytes()`, `form()` | 16 plików serwisów importuje `Body` z tauri-http |

---

## 3. Architektura Kompatybilności

### `src/utils/tauri-http.js`
Wrapper `fetch` + `Body` (text/json/bytes/form) → `@tauri-apps/plugin-http`
- **Nowa obsługa `query`**: obiekt `{ key: value }` jest konwertowany na URL search params
- Obsługuje stary format body v1: `{ type: "Form", payload: {...} }`
- Obsługuje `Body.form()`, `Body.json()`, `Body.text()`, `Body.bytes()`
- Zwraca: `{ ok, status, statusText, data }`

### `src/utils/tauri-compat.js`
Wrapper `appWindow` + `currentMonitor` → `getCurrentWindow()` z Tauri v2

### `src/utils/store.js`
`Store` z `@tauri-apps/plugin-store` v2 + `watch()` w try/catch

### `src/utils/invoke_plugin.js`
Dostarcza `utils` do wtyczek: `tauriFetch`, `http` (z Body), `readBinaryFile`, `readFile`, `readTextFile`, `Database`, `CryptoJS`, `run`

### `src-tauri/src/tray.rs`
- Tray tworzony JEDEN RAZ z unikalnym ID (`main_tray`)
- Aktualizacje menu przez `tray.set_menu()` (nie tworzy nowego tray)
- Globalny `handle_menu_event` obsługiwany przez `app.on_menu_event()`
- Menu items: `show`, `settings`, `clipboard_toggle`, `quit`

---

## 4. Build Status

```
✅ cargo check — 0 błędów (6 warningów unused imports)
✅ pnpm tauri dev — działa, HMR aktywny
```

### Jak budować

```bash
# Dev
pnpm tauri dev

# Build release
CARGO_BUILD_JOBS=1 pnpm tauri build --ignore-version-mismatches

# Uruchomienie binarki bezpośrednio
./src-tauri/target/release/pot
```

### Wymagane deps systemowe
- `libappindicator3-devel` / `libayatana-appindicator3-dev` — tray icon
- `libxdo`, `libxcb`, `libXrandr`, `tesseract-ocr` — runtime

---

## 5. Wtyczki

35 wtyczek w `/home/apawlowski/Oprogramowanie/TŁUMACZENIA/pot/plugins/`:
- **translate**: azure, baidu, caiyun, cohere, deepseek, ecdict, freedict, gcide, libre, localdeepseek-r1, moji_dict, papago, tatoeba, template, tencent, transmart, volcengine, youdao (18)
- **recognize**: baimiao, doc2x, google, microsoft, ollama, openai, paddle, qwen-ocr, rapid, template (10)
- **collection**: shanbay, template, youdao (3)
- **tts**: openai, template, volcengine, youdao (4)

Wtyczki używają `utils.tauriFetch` z compat wrappera — format odpowiedzi `{ ok, status, data }` jest zachowany.

Spakowane: `plugins.tar.gz` (209MB)

---

## 6. Znane Problemy

### 6.1 Tray — Wyjdź nie zamyka czysto
**Objaw:** Kliknięcie "Wyjdź" w tray powoduje log `"Quit requested from tray"` ale aplikacja nie zawsze się zamyka.

**Przyczyna:** `std::process::exit(0)` jest zbyt agresywne — kończy proces bez cleanup GTK event loop.

**Proponowane rozwiązanie:** Użyć `app_handle.cleanup_before_exit()` + `app.exit(0)` zamiast `std::process::exit(0)`.

### 6.2 Tray — Schowek toggle nie aktualizuje label
**Objaw:** Po kliknięciu "Schowek: WYŁĄCZONY" label nie zmienia się na "WŁĄCZONY" mimo że stan się zmienia.

**Przyczyna:** `toggle_clipboard_monitor` w `clipboard.rs` rebuilduje tray przez `update_tray()`, ale `set_menu()` nie zawsze odświeża widoczny tekst w GTK.

**Proponowane rozwiązanie:** Wymusić odświeżenie menu przez `tray.set_menu(None)` + `tray.set_menu(Some(new_menu))`.

### 6.3 Yandex API — Deprecated
**Objaw:** Yandex zwraca HTTP 400 `{"code":502,"message":"Invalid parameter: lang"}`

**Przyczyna:** Yandex Translate API v1 jest deprecated. Endpoint `translate.yandex.net/api/v1.5/tr.json/translate` nie działa poprawnie.

**Rozwiązanie:** Migracja do Yandex Translate API v2 (wymaga nowych credentials) lub usunięcie usługi.

### 6.4 Lingva — Serwer nie odpowiada
**Objaw:** `error sending request for url (https://lingva.pot-app.com/api/v1/...)`

**Przyczyna:** Serwer `lingva.pot-app.com` nie odpowiada (DNS resolve error / connection refused).

**Rozwiązanie:** Użytkownik może skonfigurować własny serwer Lingva w ustawieniach (pole "Ścieżka żądania").

### 6.5 ecdict — Language not supported
**Objaw:** `Language not supported`

**Przyczyna:** ecdict to słownik angielsko-chiński (EN↔ZH). Nie obsługuje języka polskiego.

**Rozwiązanie:** To expected behavior — nie używać ecdict dla par językowych innych niż EN-ZH.

---

## 7. TODO (opcjonalne)

- [ ] Naprawić czyste zamykanie aplikacji przez tray (`cleanup_before_exit`)
- [ ] Naprawić aktualizację label "Schowek" w tray menu
- [ ] Usunąć unused importy w Rust (6 warningów)
- [ ] Zmigrować Yandex do API v2 lub usunąć
- [ ] Dodać fallback dla niedostępnych serwerów (Lingva)
- [ ] Dodać `libappindicator3-devel` dla pełnego bundling RPM/DEB
- [ ] Przetestować 35 wtyczek po migracji

---

## 8. Logi — Potwierdzenie Działania

Z logów (`~/.local/state/com.pot-app.desktop/`):

```
[pot::tray] Building tray menu (language=pl, copy_mode=disabled)
[pot::tray] Creating new tray icon
[pot::tray] Tray menu ready

[webview][INFO] [deepl]resolve:`Option<T>` wyraźnie reprezentuje "wartość lub nic"...
[webview][INFO] [bing]resolve:"Opcja<T>" wyraźnie oznacza "wartość lub nic"...
[webview][INFO] [google]resolve:`Opcja<T>` jawnie reprezentuje „wartość albo nic"...

[pot::tray] Toggle clipboard monitor from tray
[pot::tray] Show window requested from tray
[pot::tray] Quit requested from tray
```

---

*Ostatnia aktualizacja: 22.04.2026 ~17:30*
*Migracja Tauri 1→2: UKOŃCZONA ✅*
*Dev mode: ✅ startuje, tłumaczenie DZIAŁA (DeepL + Bing + Google)*
*Tray: ✅ menu działa, exit/clipboard toggle wymagają poprawek*
*Clipboard monitoring: ✅ działa w tle*
