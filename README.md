<img width="200px" src="public/icon.svg" align="left"/>

# Pot-F Desktop - Polski Fork Pot App

> Polski fork aplikacji do tłumaczenia tekstu i OCR bazujący na [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop)

![License](https://img.shields.io/github/license/frs777/pot-f-desktop.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)
![JavaScript](https://img.shields.io/badge/-JavaScript-yellow?logo=javascript&logoColor=white)
![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white)
![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&logoColor=white)
![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white)

<br/>
<hr/>
<div align="center">

<h3>Polski | <a href='./README_EN.md'>English</a> (TODO)</h3>

# O forku

Ten projekt to fork aplikacji [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) autorstwa [Pylogmon](https://github.com/pot-app). Celem forka jest dostosowanie aplikacji do polskich użytkowników oraz systemów z rodziny Linux (w szczególności Fedora).

## Różnice względem oryginału

- Polski interfejs domyślnie
- Zależności dostosowane dla Fedory 43
- Wyłączony updater z oryginału (aktualizacje przez GitHub Releases)
- Nowa numeracja wersji (zaczynając od 0.1.0)
- Zmieniony identyfikator aplikacji na `com.pot-f-desktop.desktop`

</div>

---

<div align="center">

<table>
<tr>
    <td> <img src="asset/1.png">
    <td> <img src="asset/2.png">
    <td> <img src="asset/3.png">
</table>

# Spis treści

</div>

- [Opis](#opis)
- [Funkcje](#funkcje)
- [Obsługiwane usługi](#obsługiwane-usługi)
- [System wtyczek](#system-wtyczek)
- [Instalacja](#instalacja)
- [Wymagania systemowe](#wymagania-systemowe)
- [Wywołanie zewnętrzne](#wywołanie-zewnętrzne)
- [Obsługa Wayland](#obsługa-wayland)
- [Wielojęzyczność](#wielojęzyczność)
- [Współtwórcy](#współtwórcy)
- [Budowa z kodu](#budowa-z-kodu)
- [Podziękowania](#podziękowania)

<div align="center">

# Opis

</div>

Pot-F Desktop to aplikacja do tłumaczenia tekstu oraz OCR (optycznego rozpoznawania znaków) działająca na wszystkich głównych platformach desktopowych. Umożliwia:

| Tłumaczenie zaznaczonego tekstu | Tłumaczenie z wprowadzania | Wywołanie zewnętrzne |
| ------------------------------- | -------------------------- | -------------------- |
| Zaznacz tekst i naciśnij skrót | Naciśnij skrót, wpisz tekst i naciśnij Enter | Wywoływane przez inne aplikacje |
| <img src="asset/eg1.gif"/> | <img src="asset/eg2.gif"/> | <img src="asset/eg3.gif"/> |

| Tryb monitorowania schowka | OCR zrzutu ekranu | Tłumaczenie zrzutu ekranu |
| -------------------------- | ----------------- | ------------------------- |
| Kliknij ikonę, skopiuj tekst | Zrób zrzut i zaznacz obszar | Zrób zrzut i zaznacz obszar |
| <img src="asset/eg4.gif"/> | <img src="asset/eg5.gif"/> | <img src="asset/eg6.gif"/> |

<div align="center">

# Funkcje

</div>

- [x] Wielowątkowe tłumaczenie z wielu usług jednocześnie
- [x] Rozpoznawanie tekstu (OCR) z wielu usług
- [x] Synteza mowy (TTS)
- [x] Eksport do słowników (Anki, itd.)
- [x] Wywołanie zewnętrzne przez HTTP API
- [x] System wtyczek rozszerzający funkcjonalność
- [x] Wsparcie dla wszystkich platform PC (Windows, macOS, Linux)
- [x] Wsparcie dla Wayland (testowane na KDE, Gnome, Hyprland)
- [x] Wielojęzyczność

<div align="center">

# Obsługiwane usługi

</div>

## Tłumaczenie

- [x] [OpenAI](https://platform.openai.com/)
- [x] [智谱 AI](https://www.zhipuai.cn/)
- [x] [Gemini Pro](https://gemini.google.com/)
- [x] [Ollama](https://www.ollama.com/) (offline)
- [x] [阿里翻译](https://www.aliyun.com/product/ai/alimt)
- [x] [百度翻译](https://fanyi.baidu.com/)
- [x] [彩云小译](https://fanyi.caiyunapp.com/)
- [x] [腾讯翻译君](https://fanyi.qq.com/)
- [x] [腾讯交互翻译](https://transmart.qq.com/)
- [x] [火山翻译](https://translate.volcengine.com/)
- [x] [小牛翻译](https://niutrans.com/)
- [x] [Google](https://translate.google.com)
- [x] [Bing](https://learn.microsoft.com/zh-cn/azure/cognitive-services/translator/)
- [x] [Bing 词典](https://www.bing.com/dict)
- [x] [DeepL](https://www.deepl.com/)
- [x] [有道翻译](https://ai.youdao.com/)
- [x] [剑桥词典](https://dictionary.cambridge.org/)
- [x] [Yandex](https://translate.yandex.com/)

Więcej usług dostępnych przez [system wtyczek](#system-wtyczek)

## Rozpoznawanie tekstu (OCR)

- [x] Systemowy OCR (offline)
    - [x] [Windows.Media.OCR](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine?view=winrt-22621) na Windows
    - [x] [Apple Vision Framework](https://developer.apple.com/documentation/vision/recognizing_text_in_images) na MacOS
    - [x] [Tesseract OCR](https://github.com/tesseract-ocr) na Linux
- [x] [Tesseract.js](https://tesseract.projectnaptha.com/) (offline)
- [x] [百度](https://ai.baidu.com/tech/ocr/general)
- [x] [腾讯](https://cloud.tencent.com/product/ocr-catalog)
- [x] [火山](https://www.volcengine.com/product/OCR)
- [x] [迅飞](https://www.xfyun.cn/services/common-ocr)
- [x] [腾讯图片翻译](https://cloud.tencent.com/document/product/551/17232)
- [x] [百度图片翻译](https://fanyi-api.baidu.com/product/22)
- [x] [Simple LaTeX](https://simpletex.cn/)

Więcej usług dostępnych przez [system wtyczek](#system-wtyczek)

## Synteza mowy

- [x] [Lingva](https://github.com/thedaviddelta/lingva-translate)

Więcej usług dostępnych przez [system wtyczek](#system-wtyczek)

## Słowniki

- [x] [Anki](https://apps.ankiweb.net/)
- [x] [欧路词典](https://dict.eudic.net/)

Więcej usług dostępnych przez [system wtyczek](#system-wtyczek)

<div align="center">

# System wtyczek

</div>

Aplikacja posiada wbudowaną obsługę wielu usług, ale można ją rozszerzyć za pomocą systemu wtyczek.

## Instalacja wtyczek

Wtyczki dostępne są w formacie `.potext`. Po pobraniu pliku zainstaluj go poprzez: Preferencje -> Ustawienia usług -> Dodaj zewnętrzną wtyczkę -> Zainstaluj wtyczkę zewnętrzną.

## Rozwiązywanie problemów

- **Nie znaleziono określonego modułu (Windows)** - Brak bibliotek C++ Runtime. Zainstaluj [VC Redist](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022).
- **Nieprawidłowa aplikacja Win32 (Windows)** - Pobrano wtyczkę dla innej architektury. Pobierz właściwą wersję.

<div align="center">

# Instalacja

</div>

## Fedora 43 (RPM)

1. Pobierz paczkę `pot-f-desktop-0.1.0-1.x86_64.rpm` ze strony [Releases](https://github.com/frs777/pot-f-desktop/releases/latest)
2. Zainstaluj:
   ```bash
   sudo dnf install ./pot-f-desktop-0.1.0-1.x86_64.rpm
   ```

## Debian/Ubuntu (DEB)

1. Pobierz paczkę `pot-f-desktop_0.1.0_amd64.deb` ze strony [Releases](https://github.com/frs777/pot-f-desktop/releases/latest)
2. Zainstaluj:
   ```bash
   sudo apt-get install ./pot-f-desktop_0.1.0_amd64.deb
   ```

## Arch Linux / Manjaro

Paczka dostępna w AUR:

```bash
yay -S pot-f-desktop
# lub
paru -S pot-f-desktop
```

## Inne dystrybucje

Pobierz odpowiednią paczkę ze strony [Releases](https://github.com/frs777/pot-f-desktop/releases/latest).

<div align="center">

# Wymagania systemowe

</div>

## Wszystkie platformy

- Node.js >= 18.0.0 (tylko do budowy)
- Rust >= 1.80.0 (tylko do budowy)

## Linux

- libgtk-3-dev
- libwebkit2gtk-4.1-dev (Tauri 2.x)
- libayatana-appindicator3-dev
- librsvg2-dev
- patchelf
- libxdo-dev
- libxcb1
- libxrandr2
- libdbus-1-3
- tesseract (OCR)

## Fedora 43 - instalacja zależności

```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
```

<div align="center">

# Wywołanie zewnętrzne

</div>

Pot-F Desktop udostępnia interfejs HTTP, dzięki czemu może być wywoływany przez inne aplikacje. Domyślny port: `60828` (można zmienić w ustawieniach).

## API

```bash
POST "/" => tłumacz tekst (body: tekst do tłumaczenia)
GET "/config" => otwórz ustawienia
POST "/translate" => tłumacz tekst (jak wyżej)
GET "/selection_translate" => tłumacz zaznaczenie
GET "/input_translate" => tłumacz z wprowadzania
GET "/ocr_recognize" => OCR zrzutu ekranu
GET "/ocr_translate" => tłumaczenie zrzutu ekranu
GET "/ocr_recognize?screenshot=false" => OCR (bez wbudowanego zrzutu)
GET "/ocr_translate?screenshot=false" => tłumaczenie (bez wbudowanego zrzutu)
```

## Przykład

```bash
curl "127.0.0.1:60828/selection_translate"
```

## Użycie z własnym zrzutem ekranu

1. Zrób zrzut ekranu swoim narzędziem
2. Zapisz go w `$CACHE/com.pot-f-desktop.desktop/pot_screenshot_cut.png`
3. Wyślij żądanie: `curl "127.0.0.1:60828/ocr_recognize?screenshot=false"`

Przykład dla Flameshot (Linux):

```bash
rm ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

<div align="center">

# Obsługa Wayland

</div>

Wsparcie dla Wayland zależy od dystrybucji i środowiska. Poniżej typowe rozwiązania:

## Skróty klawiszowe nie działają

Tauri nie obsługuje skrótów na Wayland. Skonfiguruj skróty systemowe z wywołaniem przez curl (zobacz [Wywołanie zewnętrzne](#wywołanie-zewnętrzne)).

## Zrzut ekranu nie działa

Na czystym Wayland (Hyprland, itp.) użyj zewnętrznych narzędzi do zrzutu:

Przykład konfiguracji Hyprland:

```conf
bind = ALT, X, exec, grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
bind = ALT, C, exec, grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_translate?screenshot=false"
```

## Okno tłumaczenia pod kursorem (Hyprland)

```conf
windowrulev2 = float, class:(pot-f-desktop), title:(Translator|OCR|PopClip|Screenshot Translate)
windowrulev2 = move cursor 0 0, class:(pot-f-desktop), title:(Translator|PopClip|Screenshot Translate)
```

<div align="center">

# Wielojęzyczność

</div>

Aplikacja obsługuje wiele języków dzięki systemowi i18n. Domyślnym językiem jest polski.

<div align="center">

# Współtwórcy

</div>

Podziękowania dla wszystkich współtwórców oryginalnego projektu [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop).

## Budowa z kodu

### Wymagania

- Node.js >= 18.0.0
- pnpm >= 8.5.0
- Rust >= 1.80.0

### Kroki

1. Sklonuj repozytorium

    ```bash
    git clone https://github.com/frs777/pot-f-desktop.git
    ```

2. Zainstaluj zależności

    ```bash
    cd pot-f-desktop
    pnpm install
    ```

3. Zainstaluj zależności systemowe (Linux)

    ```bash
    sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
    ```

4. Tryb deweloperski

    ```bash
    pnpm tauri dev
    ```

5. Budowa paczki
    ```bash
    pnpm tauri build
    ```

<div align="center">

# Podziękowania

</div>

- [Bob](https://github.com/ripperhe/Bob) - Źródło inspiracji
- [bob-plugin-openai-translator](https://github.com/yetone/bob-plugin-openai-translator) - Referencje API OpenAI
- [@uiYzzi](https://github.com/uiYzzi) - Pomysły implementacyjne
- [@Lichenkass](https://github.com/Lichenkass) - Utrzymanie aplikacji w Deepin Store
- [Pylogmon](https://github.com/pot-app) - Autor oryginalowego projektu pot-desktop
- [Tauri](https://github.com/tauri-apps/tauri) - Świetny framework GUI

<div align="center">
