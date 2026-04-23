<img width="200px" src="public/icon.svg" align="left"/>

# Pot-F Desktop - Polish Fork of Pot App

> A Polish fork of the text translation and OCR application based on [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop)

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

<h3><a href='./README.md'>Polski</a> | English</h3>

# About the Fork

This project is a fork of [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) by [Pylogmon](https://github.com/pot-app). The goal of this fork is to adapt the application for Polish users and Linux systems (especially Fedora).

## Differences from the Original

- Polish interface by default
- Dependencies adapted for Fedora 43
- Updater from the original disabled (updates via GitHub Releases)
- New version numbering (starting from 0.1.0)
- Changed application identifier to `com.pot-f-desktop.desktop`

</div>

---

<div align="center">

<table>
<tr>
    <td> <img src="asset/1.png">
    <td> <img src="asset/2.png">
    <td> <img src="asset/3.png">
</table>

# Table of Contents

</div>

- [Description](#description)
- [Features](#features)
- [Supported Services](#supported-services)
- [Plugin System](#plugin-system)
- [Installation](#installation)
- [System Requirements](#system-requirements)
- [External Calls](#external-calls)
- [Wayland Support](#wayland-support)
- [Multilingual Support](#multilingual-support)
- [Contributors](#contributors)
- [Build from Source](#build-from-source)
- [Acknowledgements](#acknowledgements)

<div align="center">

# Description

</div>

Pot-F Desktop is a text translation and OCR (Optical Character Recognition) application that works on all major desktop platforms. It enables:

| Translate Selection | Translate by Input | External Calls |
| ------------------- | ------------------ | -------------- |
| Select text and press shortcut | Press shortcut, type text, press Enter | Called by other applications |
| <img src="asset/eg1.gif"/> | <img src="asset/eg2.gif"/> | <img src="asset/eg3.gif"/> |

| Clipboard Monitoring | Screenshot OCR | Screenshot Translation |
| -------------------- | -------------- | ---------------------- |
| Click icon, copy text | Take screenshot, select area | Take screenshot, select area |
| <img src="asset/eg4.gif"/> | <img src="asset/eg5.gif"/> | <img src="asset/eg6.gif"/> |

<div align="center">

# Features

</div>

- [x] Multi-service parallel translation
- [x] OCR from multiple services
- [x] Text-to-Speech (TTS)
- [x] Export to dictionary apps (Anki, etc.)
- [x] External calls via HTTP API
- [x] Plugin system for extended functionality
- [x] Support for all PC platforms (Windows, macOS, Linux)
- [x] Wayland support (tested on KDE, Gnome, Hyprland)
- [x] Multilingual support

<div align="center">

# Supported Services

</div>

## Translation

- [x] [OpenAI](https://platform.openai.com/)
- [x] [Zhipu AI](https://www.zhipuai.cn/)
- [x] [Gemini Pro](https://gemini.google.com/)
- [x] [Ollama](https://www.ollama.com/) (offline)
- [x] [Ali Translate](https://www.aliyun.com/product/ai/alimt)
- [x] [Baidu Translate](https://fanyi.baidu.com/)
- [x] [Caiyun](https://fanyi.caiyunapp.com/)
- [x] [Tencent Transmart](https://fanyi.qq.com/)
- [x] [Tencent Interactive Translate](https://transmart.qq.com/)
- [x] [Volcengine Translate](https://translate.volcengine.com/)
- [x] [NiuTrans](https://niutrans.com/)
- [x] [Google Translate](https://translate.google.com)
- [x] [Bing Translate](https://learn.microsoft.com/zh-cn/azure/cognitive-services/translator/)
- [x] [Bing Dictionary](https://www.bing.com/dict)
- [x] [DeepL](https://www.deepl.com/)
- [x] [Youdao](https://ai.youdao.com/)
- [x] [Cambridge Dictionary](https://dictionary.cambridge.org/)
- [x] [Yandex](https://translate.yandex.com/)

More services available via [Plugin System](#plugin-system)

## Text Recognition (OCR)

- [x] System OCR (offline)
    - [x] [Windows.Media.OCR](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine?view=winrt-22621) on Windows
    - [x] [Apple Vision Framework](https://developer.apple.com/documentation/vision/recognizing_text_in_images) on MacOS
    - [x] [Tesseract OCR](https://github.com/tesseract-ocr) on Linux
- [x] [Tesseract.js](https://tesseract.projectnaptha.com/) (offline)
- [x] [Baidu](https://ai.baidu.com/tech/ocr/general)
- [x] [Tencent](https://cloud.tencent.com/product/ocr-catalog)
- [x] [Volcengine](https://www.volcengine.com/product/OCR)
- [x] [iFlyTek](https://www.xfyun.cn/services/common-ocr)
- [x] [Tencent Image Translate](https://cloud.tencent.com/document/product/551/17232)
- [x] [Baidu Image Translate](https://fanyi-api.baidu.com/product/22)
- [x] [Simple LaTeX](https://simpletex.cn/)

More services available via [Plugin System](#plugin-system)

## Text-to-Speech

- [x] [Lingva](https://github.com/thedaviddelta/lingva-translate)

More services available via [Plugin System](#plugin-system)

## Dictionary Apps

- [x] [Anki](https://apps.ankiweb.net/)
- [x] [Eudic](https://dict.eudic.net/)

More services available via [Plugin System](#plugin-system)

<div align="center">

# Plugin System

</div>

The application has built-in support for many services, but it can be extended with the plugin system.

## Installing Plugins

Plugins are available in `.potext` format. After downloading the file, install it via: Preferences -> Service Settings -> Add External Plugin -> Install External Plugin.

## Troubleshooting

- **The specified module could not be found (Windows)** - Missing C++ Runtime libraries. Install [VC Redist](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022).
- **Not a valid Win32 application (Windows)** - Downloaded plugin for wrong architecture. Download the correct version.

<div align="center">

# Installation

</div>

## Fedora 43 (RPM)

1. Download `pot-f-desktop-0.1.0-1.x86_64.rpm` from [Releases](https://github.com/frs777/pot-f-desktop/releases/latest)
2. Install:
   ```bash
   sudo dnf install ./pot-f-desktop-0.1.0-1.x86_64.rpm
   ```

## Debian/Ubuntu (DEB)

1. Download `pot-f-desktop_0.1.0_amd64.deb` from [Releases](https://github.com/frs777/pot-f-desktop/releases/latest)
2. Install:
   ```bash
   sudo apt-get install ./pot-f-desktop_0.1.0_amd64.deb
   ```

## Arch Linux / Manjaro

Package available in AUR:

```bash
yay -S pot-f-desktop
# or
paru -S pot-f-desktop
```

## Other Distributions

Download the appropriate package from [Releases](https://github.com/frs777/pot-f-desktop/releases/latest).

<div align="center">

# System Requirements

</div>

## All Platforms

- Node.js >= 18.0.0 (build only)
- Rust >= 1.80.0 (build only)

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

## Fedora 43 - Install Dependencies

```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
```

<div align="center">

# External Calls

</div>

Pot-F Desktop provides an HTTP API for integration with other applications. Default port: `60828` (can be changed in settings).

## API

```bash
POST "/" => translate text (body: text to translate)
GET "/config" => open settings
POST "/translate" => translate text (same as above)
GET "/selection_translate" => translate selection
GET "/input_translate" => translate by input
GET "/ocr_recognize" => screenshot OCR
GET "/ocr_translate" => screenshot translation
GET "/ocr_recognize?screenshot=false" => OCR without built-in screenshot
GET "/ocr_translate?screenshot=false" => translate without built-in screenshot
```

## Example

```bash
curl "127.0.0.1:60828/selection_translate"
```

## Using with Your Own Screenshot Tool

1. Take a screenshot with your tool
2. Save it to `$CACHE/com.pot-f-desktop.desktop/pot_screenshot_cut.png`
3. Send request: `curl "127.0.0.1:60828/ocr_recognize?screenshot=false"`

Example for Flameshot (Linux):

```bash
rm ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

<div align="center">

# Wayland Support

</div>

Wayland support varies by distribution and desktop environment. Below are common solutions:

## Shortcuts Don't Work

Tauri doesn't support shortcuts on Wayland. Configure system shortcuts with curl calls (see [External Calls](#external-calls)).

## Screenshot Not Working

On pure Wayland (Hyprland, etc.) use external screenshot tools:

Hyprland configuration example:

```conf
bind = ALT, X, exec, grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
bind = ALT, C, exec, grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_translate?screenshot=false"
```

## Translation Window Following Mouse (Hyprland)

```conf
windowrulev2 = float, class:(pot-f-desktop), title:(Translator|OCR|PopClip|Screenshot Translate)
windowrulev2 = move cursor 0 0, class:(pot-f-desktop), title:(Translator|PopClip|Screenshot Translate)
```

<div align="center">

# Multilingual Support

</div>

The application supports multiple languages through the i18n system. Polish is the default language.

<div align="center">

# Contributors

</div>

Thanks to all contributors of the original [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) project.

## Build from Source

### Requirements

- Node.js >= 18.0.0
- pnpm >= 8.5.0
- Rust >= 1.80.0

### Steps

1. Clone the repository

    ```bash
    git clone https://github.com/frs777/pot-f-desktop.git
    ```

2. Install dependencies

    ```bash
    cd pot-f-desktop
    pnpm install
    ```

3. Install system dependencies (Linux)

    ```bash
    sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
    ```

4. Development mode

    ```bash
    pnpm tauri dev
    ```

5. Build package
    ```bash
    pnpm tauri build
    ```

<div align="center">

# Acknowledgements

</div>

- [Bob](https://github.com/ripperhe/Bob) - Inspiration
- [bob-plugin-openai-translator](https://github.com/yetone/bob-plugin-openai-translator) - OpenAI API Reference
- [@uiYzzi](https://github.com/uiYzzi) - Implementation ideas
- [@Lichenkass](https://github.com/Lichenkass) - Deepin Store maintenance
- [Pylogmon](https://github.com/pot-app) - Author of the original pot-desktop project
- [Tauri](https://github.com/tauri-apps/tauri) - Great GUI framework

<div align="center">
