# Pot-F Desktop - Pełna dokumentacja

> Polski fork aplikacji do tłumaczenia tekstu i OCR, bazujący na [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop)

## Spis treści

- [Czym jest Pot-F Desktop?](#czym-jest-pot-f-desktop)
- [Funkcje](#funkcje)
- [Instalacja krok po kroku](#instalacja-krok-po-kroku)
  - [Fedora 43 (RPM)](#fedora-43-rpm)
  - [Debian/Ubuntu (DEB)](#debianubuntu-deb)
  - [Arch Linux / Manjaro](#arch-linux--manjaro)
- [Zależności](#zależności)
  - [Fedora 43](#fedora-43)
  - [Debian/Ubuntu](#debianubuntu)
- [Pierwsze uruchomienie](#pierwsze-uruchomienie)
- [Konfiguracja](#konfiguracja)
- [Rozwiązywanie problemów](#rozwiązywanie-problemów)
- [Lista zmian względem oryginału](#lista-zmian-względem-oryginału)
- [Budowa z kodu źródłowego](#budowa-z-kodu-źródłowego)

---

## Czym jest Pot-F Desktop?

Pot-F Desktop to aplikacja desktopowa do tłumaczenia tekstu oraz OCR (optycznego rozpoznawania znaków). Jest to fork projektu [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) autorstwa Pylogmon, dostosowany specjalnie dla polskich użytkowników i systemów Linux (w szczególności Fedory).

### Główne cechy forka

- **Polski interfejs domyślnie** - aplikacja startuje z polskim językiem
- **Nowy identyfikator** - `com.pot-f-desktop.desktop` (nie koliduje z oryginałem)
- **Zależności dla Fedory** - paczki RPM budowane z myślą o Fedora 43
- **Wyłączony updater oryginału** - aktualizacje przez GitHub Releases

---

## Funkcje

- Wielowątkowe tłumaczenie z wielu usług jednocześnie (Google, DeepL, Bing, OpenAI, Gemini, i wiele innych)
- Rozpoznawanie tekstu ze zdjęć (OCR) - Tesseract, Baidu, Tencent, i inne
- Synteza mowy (TTS)
- Eksport słówek do Anki i innych słowników
- Monitorowanie schowka - kopiuj tekst, a aplikacja automatycznie go przetłumaczy
- Tłumaczenie zrzutów ekranu
- System wtyczek rozszerzający funkcjonalność
- Wywołanie zewnętrzne przez HTTP API (możliwość integracji z innymi aplikacjami)
- Wsparcie dla Wayland

---

## Instalacja krok po kroku

### Fedora 43 (RPM)

1. **Pobierz paczkę RPM**
   Przejdź do strony [Releases](https://github.com/frs777/pot-f-desktop/releases/latest) i pobierz plik:
   ```
   pot-f-desktop-0.1.0-1.x86_64.rpm
   ```

2. **Zainstaluj zależności** (jeśli nie zostały automatycznie rozwiązane):
   ```bash
   sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
   ```

3. **Zainstaluj aplikację**:
   ```bash
   sudo dnf install ./pot-f-desktop-0.1.0-1.x86_64.rpm
   ```

4. **Uruchom aplikację** z menu aplikacji lub z terminala:
   ```bash
   pot-f
   ```

### Debian/Ubuntu (DEB)

1. **Pobierz paczkę DEB**
   Przejdź do strony [Releases](https://github.com/frs777/pot-f-desktop/releases/latest) i pobierz plik:
   ```
   pot-f-desktop_0.1.0_amd64.deb
   ```

2. **Zainstaluj zależności**:
   ```bash
   sudo apt-get install -y libgtk-3-0 libwebkit2gtk-4.1-0 libayatana-appindicator3-1 librsvg2-2 patchelf xdotool libxcb1 libxrandr2 libdbus-1-3 tesseract-ocr
   ```

3. **Zainstaluj aplikację**:
   ```bash
   sudo apt-get install ./pot-f-desktop_0.1.0_amd64.deb
   ```

4. **Uruchom aplikację**:
   ```bash
   pot-f
   ```

### Arch Linux / Manjaro

Paczka dostępna w AUR jako `pot-f-desktop`:

```bash
yay -S pot-f-desktop
# lub
paru -S pot-f-desktop
```

---

## Zależności

### Fedora 43

| Zależność | Opis |
|-----------|------|
| gtk3-devel | Biblioteka GUI |
| webkit2gtk4.1-devel | Silnik przeglądarki (WebView) |
| libayatana-appindicator-gtk3-devel | Ikona w trayu |
| librsvg2-devel | Obsługa grafiki SVG |
| patchelf | Narzędzie do edycji ELF |
| xdotool-devel | Symulacja klawiatury/myszy |
| libxcb-devel | X11 |
| libXrandr2 | Rozdzielczość ekranu |
| dbus-devel | Komunikacja międzyprocesowa |
| tesseract | Silnik OCR |

Instalacja jednym poleceniem:
```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
```

### Debian/Ubuntu

| Zależność | Opis |
|-----------|------|
| libgtk-3-0 | Biblioteka GUI |
| libwebkit2gtk-4.1-0 | Silnik przeglądarki (WebView) |
| libayatana-appindicator3-1 | Ikona w trayu |
| librsvg2-2 | Obsługa grafiki SVG |
| patchelf | Narzędzie do edycji ELF |
| xdotool | Symulacja klawiatury/myszy |
| libxcb1 | X11 |
| libxrandr2 | Rozdzielczość ekranu |
| libdbus-1-3 | Komunikacja międzyprocesowa |
| tesseract-ocr | Silnik OCR |

Instalacja jednym poleceniem:
```bash
sudo apt-get install -y libgtk-3-0 libwebkit2gtk-4.1-0 libayatana-appindicator3-1 librsvg2-2 patchelf xdotool libxcb1 libxrandr2 libdbus-1-3 tesseract-ocr
```

---

## Pierwsze uruchomienie

Po pierwszym uruchomieniu aplikacja pojawi się w zasobniku systemowym (tray). Domyślnie:

1. **Język interfejsu**: Polski
2. **Domyślne usługi tłumaczenia**: Należy skonfigurować w ustawieniach
3. **Skróty klawiszowe**: Należy ustawić w Preferencjach -> Skróty klawiszowe
4. **Katalog konfiguracji**: `~/.config/com.pot-f-desktop.desktop/`

### Konfiguracja usług tłumaczenia

1. Otwórz ustawienia (kliknij prawym na ikonę w trayu -> Preferencje)
2. Przejdź do zakładki "Usługi tłumaczenia"
3. Dodaj żądane usługi (Google, DeepL, OpenAI, itp.)
4. Wprowadź klucze API jeśli wymagane
5. Zapisz ustawienia

### Konfiguracja OCR

1. Otwórz ustawienia
2. Przejdź do zakładki "Usługi OCR"
3. Na Linuxie domyślnie dostępny jest Tesseract
4. Dodaj dodatkowe usługi OCR jeśli potrzebujesz

---

## Konfiguracja

### Skróty klawiszowe

W ustawieniach możesz skonfigurować skróty dla:
- Tłumaczenie zaznaczonego tekstu
- Tłumaczenie z wprowadzania
- OCR zrzutu ekranu
- Tłumaczenie zrzutu ekranu

> **Uwaga dla Wayland**: Skróty w aplikacji mogą nie działać na Wayland. Skonfiguruj skróty systemowe z wywołaniem HTTP API.

### Monitor schowka

Włącz monitor schowka, aby automatycznie tłumaczyć kopiowany tekst:
1. Ustawienia -> Ogólne -> Monitor schowka
2. Włącz opcję
3. Aplikacja będzie tłumaczyć tekst po skopiowaniu do schowka

### Integracja z innymi aplikacjami

Pot-F Desktop udostępnia API HTTP na porcie `60828` (domyślnie). Możesz wywoływać funkcje aplikacji z innych programów:

```bash
# Tłumaczenie zaznaczenia
curl "127.0.0.1:60828/selection_translate"

# Tłumaczenie tekstu
curl -X POST -d "tekst do tłumaczenia" "127.0.0.1:60828/translate"

# Otwarcie ustawień
curl "127.0.0.1:60828/config"
```

---

## Rozwiązywanie problemów

### Aplikacja nie uruchamia się

1. **Sprawdź zależności**: Upewnij się, że wszystkie zależności są zainstalowane
2. **Sprawdź WebView2** (Windows): Upewnij się, że WebView2 jest zainstalowany
3. **Uruchom z terminala**: `pot-f` - sprawdź komunikaty błędów

### Skróty klawiszowe nie działają (Linux/Wayland)

Tauri nie obsługuje skrótów na Wayland. Rozwiązanie:
1. Skonfiguruj skróty w swoim środowisku graficznym
2. Ustaw wywołanie przez curl, np.:
   ```bash
   curl "127.0.0.1:60828/selection_translate"
   ```

### OCR nie działa (Tesseract)

1. Upewnij się, że tesseract jest zainstalowany:
   ```bash
   tesseract --version
   ```
2. Zainstaluj pakiety językowe:
   ```bash
   # Fedora
   sudo dnf install tesseract-langpack-pol
   # Debian/Ubuntu
   sudo apt-get install tesseract-ocr-pol
   ```

### Zrzut ekranu nie działa (Wayland)

Na Wayland wbudowany zrzut ekranu może nie działać. Użyj zewnętrznych narzędzi:

**Flameshot**:
```bash
rm ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

**Grim + Slurp** (Hyprland/Sway):
```bash
grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

### Ikona w trayu nie pojawia się

Upewnij się, że zainstalowałeś `libayatana-appindicator` (lub odpowiednik dla swojej dystrybucji). Niektóre środowiska mogą wymagać dodatkowych rozszerzeń do obsługi tray.

### Aplikacja nie łączy się z usługami tłumaczenia

1. Sprawdź połączenie internetowe
2. Sprawdź czy klucze API są poprawne
3. Sprawdź czy firewall nie blokuje połączenia

---

## Lista zmian względem oryginału

### Wersja 0.1.0

- Zmieniono nazwę z `pot-desktop` na `pot-f-desktop`
- Zmieniono nazwę binarki z `pot` na `pot-f`
- Nowy identyfikator: `com.pot-app.desktop` -> `com.pot-f-desktop.desktop`
- Polski interfejs jako domyślny
- Wyłączono updater z oryginału (`plugins.updater.active: false`)
- Zmieniono zależność RPM z `tesseract-ocr` na `tesseract` (nazwa w Fedorze)
- Zaktualizowano linki w stronie "O programie"
- Dodano informację o forku w sekcji About
- Dodano polskie tłumaczenie interfejsu
- Zmieniono numerację wersji na 0.1.0
- Dodano README.md, README_PL.md, CHANGELOG.md, CONTRIBUTING.md
- Dodano pole `repository` w package.json

---

## Budowa z kodu źródłowego

### Wymagania wstępne

| Narzędzie | Wersja | Opis |
|-----------|--------|------|
| Node.js | >= 18.0.0 | Środowisko JavaScript |
| pnpm | >= 8.5.0 | Menadżer pakietów |
| Rust | >= 1.80.0 | Kompilator |
| Tauri CLI | najnowsza | Narzędzie do budowy |

### Instalacja narzędzi

#### Fedora

```bash
# Node.js (jeśli nie masz)
sudo dnf install nodejs

# pnpm
npm install -g pnpm

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Tauri CLI
cargo install tauri-cli --version "^2"
```

#### Debian/Ubuntu

```bash
# Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# pnpm
npm install -g pnpm

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Tauri CLI
cargo install tauri-cli --version "^2"
```

### Budowa

1. **Sklonuj repozytorium**:
   ```bash
   git clone https://github.com/frs777/pot-f-desktop.git
   cd pot-f-desktop
   ```

2. **Zainstaluj zależności JavaScript**:
   ```bash
   pnpm install
   ```

3. **Zainstaluj zależności systemowe** (Linux):
   ```bash
   # Fedora
   sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract

   # Debian/Ubuntu
   sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev patchelf xdotool libxcb1 libxrandr2 libdbus-1-3 tesseract-ocr
   ```

4. **Tryb deweloperski** (uruchomienie bez budowania paczki):
   ```bash
   pnpm tauri dev
   ```

5. **Budowa paczki instalacyjnej**:
   ```bash
   pnpm tauri build
   ```

   Po zbudowaniu paczki znajdziesz w katalogu `src-tauri/target/release/bundle/`:
   - `rpm/` - paczka RPM dla Fedory
   - `deb/` - paczka DEB dla Debiana/Ubuntu

---

## Kontakt

- **GitHub**: https://github.com/frs777/pot-f-desktop
- **Issues**: https://github.com/frs777/pot-f-desktop/issues
- **Email wsparcia**: frss@protonmail.com
- **Dyskusje**: https://github.com/frs777/pot-f-desktop/discussions

---

## Licencja

Ten projekt jest dostępny na licencji GPL-3.0 (zachowana z oryginału).
Oryginalny projekt: [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) autorstwa [Pylogmon](https://github.com/pot-app).
