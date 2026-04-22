<img width="200px" src="public/icon.svg" align="left"/>

# Pot (派了个萌的翻译器)

> 🌈 Wieloplatformowe oprogramowanie do tłumaczenia ([QQ Kanał](https://pd.qq.com/s/akns94e1r))

![License](https://img.shields.io/github/license/pot-app/pot-desktop.svg)
![Tauri](https://img.shields.io/badge/Tauri-1.6.8-blue?logo=tauri)
![JavaScript](https://img.shields.io/badge/-JavaScript-yellow?logo=javascript&logoColor=white)
![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white)
![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&logoColor=white)
![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white)

<br/>
<hr/>
<div align="center">

<h3>中文 | <a href='./README_EN.md'>English</a> | <a href='./README_KR.md'>한글</a> | <a href='./README_PL.md'>Polski</a></h3>

<table>
<tr>
    <td> <img src="asset/1.png">
    <td> <img src="asset/2.png">
    <td> <img src="asset/3.png">
</table>

# Spis treści

</div>

-   [Instrukcja obsługi](#instrukcja-obługi)
-   [Funkcje](#funkcje)
-   [Obsługiwane interfejsy](#obsługiwane-interfejsy)
-   [System wtyczek](#system-wtyczek)
-   [Przewodnik instalacji](#przewodnik-instalacji)
-   [Wywołania zewnętrzne](#wywołania-zewnętrzne)
-   [Wsparcie Wayland](#wsparcie-wayland)
-   [Internacjonalizacja](#internacjonalizacjaweblate)
-   [Współtwórcy](#współtwórcy)
-   [Podziękowania](#podziękowania)

<div align="center">

# Instrukcja obsługi

| Tłumaczenie zaznaczenia                              | Tłumaczenie tekstu                                              | Wywołania zewnętrzne                                                    |
| ---------------------------------------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------- |
| Zaznacz tekst do tłumaczenia i naciśnij skonfigurowany skrót do tłumaczenia zaznaczenia | Naciśnij skrót do tłumaczenia tekstu, aby otworzyć okno tłumaczenia, wprowadź tekst i naciśnij Enter | Wywoływane przez inne oprogramowanie dla wygodniejszych funkcji, zobacz [Wywołania zewnętrzne](#wywołania-zewnętrzne) |
| <img src="asset/eg1.gif"/>                           | <img src="asset/eg2.gif"/>                                     | <img src="asset/eg3.gif"/>                                           |

| Tryb monitorowania schowka                                                 | OCR zrzutu ekranu                                          | Tłumaczenie zrzutu ekranu                                         |
| ---------------------------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------ |
| Kliknij ikonę w lewym górnym rogu dowolego panelu tłumaczenia, aby uruchomić monitorowanie schowka, skopiuj tekst, aby przetłumaczyć | Naciśnij skrót OCR zrzutu ekranu, zaznacz obszar do rozpoznania | Naciśnij skrót tłumaczenia zrzutu ekranu, zaznacz obszar do tłumaczenia |
| <img src="asset/eg4.gif"/>                                             | <img src="asset/eg5.gif"/>                        | <img src="asset/eg6.gif"/>                       |

</div>

<div align="center">

# Funkcje

</div>

-   [x] Równoległe tłumaczenie wieloma interfejsami ([Obsługiwane interfejsy](#obsługiwane-interfejsy))
-   [x] Rozpoznawanie tekstu wieloma interfejsami ([Obsługiwane interfejsy](#obsługiwane-interfejsy))
-   [x] Synteza mowy wieloma interfejsami ([Obsługiwane interfejsy](#obsługiwane-interfejsy))
-   [x] Eksport do fiszek ([Obsługiwane interfejsy](#obsługiwane-interfejsy))
-   [x] Wywołania zewnętrzne ([Szczegóły](#wywołania-zewnętrzne))
-   [x] Obsługa systemu wtyczek ([System wtyczek](#system-wtyczek))
-   [x] Obsługa wszystkich platform PC (Windows, macOS, Linux)
-   [x] Obsługa Wayland (testowane na KDE, Gnome oraz Hyprland)
-   [x] Wsparcie wielojęzyczne

<div align="center">

# Obsługiwane interfejsy

</div>

## Tłumaczenie

-   [x] [OpenAI](https://platform.openai.com/)
-   [x] [Zhipu AI](https://www.zhipuai.cn/)
-   [x] [Gemini Pro](https://gemini.google.com/)
-   [x] [Ollama](https://www.ollama.com/) (offline)
-   [x] [Alibaba Translation](https://www.aliyun.com/product/ai/alimt)
-   [x] [Baidu Translation](https://fanyi.baidu.com/)
-   [x] [Caiyun](https://fanyi.caiyunapp.com/)
-   [x] [Tencent Translation](https://fanyi.qq.com/)
-   [x] [Tencent TranSmart](https://transmart.qq.com/)
-   [x] [VolcEngine Translation](https://translate.volcengine.com/)
-   [x] [NiuTrans](https://niutrans.com/)
-   [x] [Google](https://translate.google.com)
-   [x] [Bing](https://learn.microsoft.com/zh-cn/azure/cognitive-services/translator/)
-   [x] [Bing Dictionary](https://www.bing.com/dict)
-   [x] [DeepL](https://www.deepl.com/)
-   [x] [Youdao Translation](https://ai.youdao.com/)
-   [x] [Cambridge Dictionary](https://dictionary.cambridge.org/)
-   [x] [Yandex](https://translate.yandex.com/)
-   [x] [Lingva](https://github.com/TheDavidDelta/lingva-translate) ([wtyczka](https://github.com/pot-app/pot-app-translate-plugin-template))
-   [x] [Tatoeba](https://tatoeba.org/) ([wtyczka](https://github.com/pot-app/pot-app-translate-plugin-tatoeba))
-   [x] [ECDICT](https://github.com/skywind3000/ECDICT) ([wtyczka](https://github.com/pot-app/pot-app-translate-plugin-ecdict))

Więcej obsługiwanych interfejsów w [System wtyczek](#system-wtyczek)

## Rozpoznawanie tekstu

-   [x] Systemowy OCR (offline)
    -   [x] [Windows.Media.OCR](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine?view=winrt-22621) na Windows
    -   [x] [Apple Vision Framework](https://developer.apple.com/documentation/vision/recognizing_text_in_images) na MacOS
    -   [x] [Tesseract OCR](https://github.com/tesseract-ocr) na Linux
-   [x] [Tesseract.js](https://tesseract.projectnaptha.com/) (offline)
-   [x] [Baidu](https://ai.baidu.com/tech/ocr/general)
-   [x] [Tencent](https://cloud.tencent.com/product/ocr-catalog)
-   [x] [VolcEngine](https://www.volcengine.com/product/OCR)
-   [x] [iFlyTek](https://www.xfyun.cn/services/common-ocr)
-   [x] [Tencent Image Translation](https://cloud.tencent.com/document/product/551/17232)
-   [x] [Baidu Image Translation](https://fanyi-api.baidu.com/product/22)
-   [x] [Simple LaTeX](https://simpletex.cn/)
-   [x] [OCRSpace](https://ocr.space/) ([wtyczka](https://github.com/pot-app/pot-app-recognize-plugin-template))
-   [x] [Rapid](https://github.com/RapidAI/RapidOcrOnnx) (offline [wtyczka](https://github.com/pot-app/pot-app-recognize-plugin-rapid))
-   [x] [Paddle](https://github.com/hiroi-sora/PaddleOCR-json) (offline [wtyczka](https://github.com/pot-app/pot-app-recognize-plugin-paddle))

Więcej obsługiwanych interfejsów w [System wtyczek](#system-wtyczek)

## Synteza mowy

-   [x] [Lingva](https://github.com/thedaviddelta/lingva-translate)

Więcej obsługiwanych interfejsów w [System wtyczek](#system-wtyczek)

## Fiszki

-   [x] [Anki](https://apps.ankiweb.net/)
-   [x] [Eudic](https://dict.eudic.net/)
-   [x] [Youdao](https://www.youdao.com/) ([wtyczka](https://github.com/pot-app/pot-app-collection-plugin-youdao))
-   [x] [Shanbay](https://web.shanbay.com/web/main) ([wtyczka](https://github.com/pot-app/pot-app-collection-plugin-shanbay))

Więcej obsługiwanych interfejsów w [System wtyczek](#system-wtyczek)

<div align="center">

# System wtyczek

</div>

Wbudowane interfejsy są ograniczone, ale możesz rozszerzyć funkcjonalność oprogramowania za pomocą systemu wtyczek.

## Instalacja wtyczek

Możesz znaleźć potrzebne wtyczki na [Liście wtyczek](https://pot-app.com/plugin.html), a następnie pobrać je z repozytorium wtyczek.

Rozszerzenie wtyczek pot to `.potext`. Po pobraniu pliku `.potext`, wybierz Preferencje - Ustawienia usług - Dodaj zewnętrzną wtyczkę - Zainstaluj zewnętrzną wtyczkę i wybierz odpowiedni plik `.potext`, aby pomyślnie zainstalować. Po dodaniu do listy usług może być używana jak wbudowana usługa.

### Rozwiązywanie problemów

-   Nie można znaleźć określonego modułu (Windows)

    Taki błąd występuje, ponieważ systemowi brakuje bibliotek C++. Zainstaluj [tutaj](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022), aby rozwiązać problem.

-   Nieprawidłowa aplikacja Win32 (Windows)

    Taki błąd oznacza, że nie pobrałeś właściwej wtyczki dla swojego systemu lub architektury. Pobierz właściwą wtyczkę z repozytorium wtyczek, aby rozwiązać problem.

## Tworzenie wtyczek

W sekcji [Szablony](https://pot-app.com/plugin.html#%E6%A8%A1%E6%9D%BF) na [Liście wtyczek](https://pot-app.com/plugin.html) dostępne są szablony tworzenia różnych wtyczek. Dokumentację tworzenia znajdziesz w odpowiednich repozytoriach szablonów.

<div align="center">

# Przewodnik instalacji

</div>

## Windows

### Instalacja przez Winget

```powershell
winget install Pylogmon.pot
```

### Ręczna instalacja

1. Pobierz najnowszą paczkę `exe` ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest).

    - Dla maszyn 64-bitowych pobierz `pot_{version}_x64-setup.exe`
    - Dla maszyn 32-bitowych pobierz `pot_{version}_x86-setup.exe`
    - Dla maszyn arm64 pobierz `pot_{version}_arm64-setup.exe`

2. Kliknij dwukrotnie paczkę, aby zainstalować.

### Rozwiązywanie problemów

-   Brak interfejsu po uruchomieniu, kliknięcie ikony zasobnika nie reaguje

    Sprawdź, czy WebView2 nie został odinstalowany/wyłączony. Jeśli tak, zainstaluj ręcznie WebView2 lub przywróć go.

    Jeśli system firmowy nie pozwala na instalację, pobierz wersję z wbudowanym WebView2 ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest) `pot_{version}_{arch}_fix_webview2_runtime-setup.exe`

    Jeśli problem nadal występuje, spróbuj uruchomić w trybie zgodności z Windows7.

## MacOS

### Instalacja przez Brew

1. Dodaj nasz tap:

```bash
brew tap pot-app/homebrew-tap
```

2. Zainstaluj pot:

```bash
brew install --cask pot
```

3. Aktualizuj pot:

```bash
brew upgrade --cask pot
```

### Ręczna instalacja

1. Pobierz najnowszą paczkę `dmg` ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest). (Jeśli używasz chipa M1, pobierz paczkę `pot_{version}_aarch64.dmg`, w przeciwnym razie pobierz `pot_{version}_x64.dmg`)
2. Kliknij dwukrotnie pobrany plik i przeciągnij pot do folderu Aplikacje, aby zakończyć instalację.

### Rozwiązywanie problemów

-   Nie można otworzyć "pot", ponieważ deweloper nie może zostać zweryfikowany.

    Kliknij Anuluj, a następnie przejdź do Ustawienia -> Prywatność i bezpieczeństwo, kliknij Otwórz mimo to, a następnie kliknij Otwórz w wyskakującym oknie. Od tej pory otwieranie pot nie będzie wyświetlać żadnych ostrzeżeń.

    Jeśli nie możesz znaleźć tej opcji w Prywatność i bezpieczeństwo lub pojawia się komunikat o uszkodzonym pliku. Otwórz Terminal.app i wprowadź poniższe polecenie, a następnie uruchom ponownie pot:

    ```bash
    sudo xattr -d com.apple.quarantine /Applications/pot.app
    ```

-   Jeśli za każdym razem pojawia się komunikat o uprawnieniach dostępu lub nie można tłumaczyć zaznaczenia, przejdź do Ustawienia -> Prywatność i bezpieczeństwo -> Ułatwienia dostępu, usuń "pot" i dodaj ponownie "pot".

## Linux

### Debian/Ubuntu

1. Pobierz najnowszą paczkę `deb` ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest).

2. Zainstaluj używając `apt-get`

    ```bash
    sudo apt-get install ./pot_{version}_amd64.deb
    ```

### Arch/Manjaro

> [!WARNING]
> W najnowszej wersji [Webkit2Gtk](https://archlinux.org/packages/extra/x86_64/webkit2gtk) (2.42.0) z powodu niepełnej implementacji DMABUF przez własnościowe sterowniki Nvidia wystąpią problemy z uruchomieniem i awarie.<br>
> Zainstaluj starszą wersję lub dodaj zmienną środowiskową `WEBKIT_DISABLE_DMABUF_RENDERER=1` w `/etc/environment` (lub innym miejscu konfiguracji środowiska), aby wyłączyć DMABUF.

1. Sprawdź w [AUR](https://aur.archlinux.org/packages?O=0&K=pot-translation)

Zainstaluj używając `AUR helper`:

```bash
yay -S pot-translation # lub pot-translation-bin

# paru -S pot-translation # lub pot-translation-bin
```

2. Jeśli używasz źródła `archlinuxcn`, możesz zainstalować bezpośrednio używając pacman:

```bash
sudo pacman -S pot-translation
```

### Flatpak

> [!WARNING]
> Wersja Flatpak nie ma ikony zasobnika.

<a href='https://flathub.org/apps/com.pot_app.pot'>
    <img width='240' alt='Pobierz z Flathub' src='https://flathub.org/api/badge?locale=pl'/>
</a>

<div align="center">

# Wywołania zewnętrzne

</div>

Pot zapewnia pełne interfejsy HTTP, dzięki czemu może być wywoływany przez inne oprogramowanie. Możesz wywołać pot, wysyłając żądanie HTTP do `127.0.0.1:port`, gdzie `port` to port nasłuchiwania pot, domyślnie `60828`, który można zmienić w ustawieniach oprogramowania.

## Dokumentacja API:

```bash
POST "/" => Tłumacz określony tekst (body to tekst do tłumaczenia),
GET "/config" => Otwórz ustawienia,
POST "/translate" => Tłumacz określony tekst (jak "/"),
GET "/selection_translate" => Tłumaczenie zaznaczenia,
GET "/input_translate" => Tłumaczenie tekstu,
GET "/ocr_recognize" => OCR zrzutu ekranu,
GET "/ocr_translate" => Tłumaczenie zrzutu ekranu,
GET "/ocr_recognize?screenshot=false" => OCR zrzutu ekranu (bez wbudowanego zrzutu ekranu),
GET "/ocr_translate?screenshot=false" => Tłumaczenie zrzutu ekranu (bez wbudowanego zrzutu ekranu),
GET "/ocr_recognize?screenshot=true" => OCR zrzutu ekranu,
GET "/ocr_translate?screenshot=true" => Tłumaczenie zrzutu ekranu,
```

## Przykład:

-   Wywołanie tłumaczenia zaznaczenia:

    Aby wywołać tłumaczenie zaznaczenia pot, wystarczy wysłać żądanie do `127.0.0.1:port`.

    Na przykład używając curl:

    ```bash
    curl "127.0.0.1:60828/selection_translate"
    ```

## Bez wbudowanego zrzutu ekranu

Ta funkcja umożliwia wywołanie funkcji OCR zrzutu ekranu/tłumaczenia zrzutu ekranu bez używania wbudowanego zrzutu ekranu, dzięki czemu możesz użyć ulubionego narzędzia do zrzutów ekranu i rozwiązać problem z niedziałającym wbudowanym zrzutem ekranu na niektórych platformach.

### Proces wywołania

1. Zrób zrzut ekranu innym narzędziem
2. Zapisz zrzut ekranu w `$CACHE/com.pot-app.desktop/pot_screenshot_cut.png`
3. Wyślij żądanie do `127.0.0.1:port/ocr_recognize?screenshot=false`, aby wywołać

> `$CACHE` to katalog systemowy cache, np. na Windows to `C:\Users\{nazwa_użytkownika}\AppData\Local\com.pot-app.desktop\pot_screenshot_cut.png`

### Przykład

Użycie Flameshot do OCR zrzutu ekranu na Linuxie:

```bash
rm ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

## Istniejące użycie (szybkie tłumaczenie zaznaczenia)

### SnipDo (Windows)

1. Pobierz i zainstaluj SnipDo ze [Sklepu Microsoft](https://apps.microsoft.com/store/detail/snipdo/9NPZ2TVKJVT7).
2. Pobierz rozszerzenie SnipDo dla pot (pot.pbar) ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest).
3. Kliknij dwukrotnie pobrany plik rozszerzenia, aby zainstalować.
4. Zaznacz tekst, zobaczysz pasek narzędzi SnipDo, kliknij przycisk tłumaczenia, aby przetłumaczyć.

### PopClip (MacOS)

1. Pobierz i zainstaluj PopClip ze [Sklepu App Store](https://apps.apple.com/us/app/popclip/id445189367?mt=12).
2. Pobierz rozszerzenie PopClip dla pot (pot.popclipextz) ze strony [Wydania](https://github.com/pot-app/pot-desktop/releases/latest).
3. Kliknij dwukrotnie pobrany plik rozszerzenia, aby zainstalować.
4. Włącz rozszerzenie pot w PopClip, zaznacz tekst, aby przetłumaczyć.

### Starry (Linux)

> Starry jest wciąż w fazie rozwoju, więc możesz go tylko ręcznie skompilować.

Github: [ccslykx/Starry](https://github.com/ccslykx/Starry)

<div align="center">

# Wsparcie Wayland

</div>

Ze względu na różne poziomy wsparcia dla Wayland przez różne dystrybucje, sam pot nie może zapewnić idealnego wsparcia. Oto rozwiązania niektórych typowych problemów. Po odpowiedniej konfiguracji pot może działać idealnie na Wayland.

## Skróty klawiszowe nie działają

Ponieważ Tauri nie obsługuje Wayland w schemacie skrótów klawiszowych, skróty klawiszowe w aplikacji pot nie działają na Wayland. Możesz skonfigurować skrót systemowy do wysyłania żądań curl do wywołania pot, zobacz [Wywołania zewnętrzne](#wywołania-zewnętrzne)

## Zrzut ekranu nie działa

Na niektórych środowiskach pulpitu/menedżerach okien Wayland (np. Hyprland) wbudowany zrzut ekranu pot nie działa. Możesz użyć innych narzędzi do zrzutów ekranu, zobacz [Bez wbudowanego zrzutu ekranu](#bez-wbudowanego-zrzutu-ekranu)

Przykład konfiguracji dla Hyprland (używając grim i slurp do zrzutu ekranu):

```conf
bind = ALT, X, exec, grim -g "$(slurp)" ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
bind = ALT, C, exec, grim -g "$(slurp)" ~/.cache/com.pot-app.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_translate?screenshot=false"
```

Inne środowiska pulpitu/menedżery okien działają podobnie.

## Okno tłumaczenia zaznaczenia podąża za kursorem myszy

Ponieważ pot nie może jeszcze uzyskać poprawnych koordynatów myszy na Wayland, wewnętrzna implementacja nie działa. Dla niektórych środowisk pulpitu/menedżerów okien można skonfigurować reguły okna, aby okno podążało za kursorem myszy. Przykład dla Hyprland:

```conf
windowrulev2 = float, class:(pot), title:(Translator|OCR|PopClip|Screenshot Translate) # Okno tłumaczenia pływające
windowrulev2 = move cursor 0 0, class:(pot), title:(Translator|PopClip|Screenshot Translate) # Okno tłumaczenia podąża za kursorem myszy
```

<div align="center">

# Internacjonalizacja([Weblate](https://hosted.weblate.org/engage/pot-app/))

[![](https://hosted.weblate.org/widget/pot-app/pot-desktop/svg-badge.svg)](https://hosted.weblate.org/engage/pot-app/)

[![](https://hosted.weblate.org/widget/pot-app/pot-desktop/pl/multi-auto.svg)](https://hosted.weblate.org/engage/pot-app/)

</div>

<div align="center">

# Współtwórcy

</div>

<img src="https://github.com/pot-app/.github/blob/master/pot-desktop-contributions.svg?raw=true" width="100%"/>

## Ręczna kompilacja

### Wymagania środowiskowe

Node.js >= 18.0.0

pnpm >= 8.5.0

Rust >= 1.80.0

### Rozpoczęcie kompilacji

1. Sklonuj repozytorium

    ```bash
    git clone https://github.com/pot-app/pot-desktop.git
    ```

2. Zainstaluj zależności

    ```bash
    cd pot-desktop
    pnpm install
    ```

3. Zainstaluj zależności (tylko Linux)

    ```bash
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev patchelf libxdo-dev libxcb1 libxrandr2 libdbus-1-3
    ```

4. Tryb deweloperski

    ```bash
    pnpm tauri dev # Uruchom aplikację w trybie deweloperskim
    ```

5. Budowanie
    ```bash
    pnpm tauri build # Buduj paczkę instalacyjną
    ```

<div align="center">

# Podziękowania

</div>

-   [Bob](https://github.com/ripperhe/Bob) Źródło inspiracji
-   [bob-plugin-openai-translator](https://github.com/yetone/bob-plugin-openai-translator) Referencje interfejsu OpenAI
-   [@uiYzzi](https://github.com/uiYzzi) Pomysły implementacji
-   [@Lichenkass](https://github.com/Lichenkass) Utrzymanie aplikacji pot w sklepie Deepin
-   [Tauri](https://github.com/tauri-apps/tauri) Świetny framework GUI

<div align="center">
