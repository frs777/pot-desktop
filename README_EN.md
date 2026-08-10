# Pot-F Desktop — Full Documentation

[Polski](README.md) | **English**

> A Polish fork of the text translation and OCR application based on [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop).

## Table of contents

- [What is Pot-F Desktop?](#what-is-pot-f-desktop)
- [Features](#features)
- [Step-by-step installation](#step-by-step-installation)
- [Dependencies](#dependencies)
- [First launch](#first-launch)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Changes from the original project](#changes-from-the-original-project)
- [Building from source](#building-from-source)
- [Contact](#contact)
- [License](#license)

---

## What is Pot-F Desktop?

Pot-F Desktop is a desktop application for text translation and OCR (optical character recognition). It is a fork of [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) by Pylogmon, adapted for Polish users and Linux systems, especially Fedora.

### Main characteristics of the fork

- **Polish interface by default** — the application starts in Polish.
- **New application identifier** — `com.pot-f-desktop.desktop`, so it does not conflict with the original application.
- **Fedora dependencies** — RPM packages are built with Fedora 43 in mind.
- **Original updater disabled** — Pot-F updates are distributed through GitHub Releases.

---

## Features

- Parallel translation using multiple services, including Google, DeepL, Bing, OpenAI, Gemini, and others.
- Image text recognition (OCR) with Tesseract, Baidu, Tencent, and other services.
- Text-to-speech synthesis (TTS).
- Vocabulary export to Anki and other collections.
- Clipboard monitoring and automatic translation of copied text.
- Screenshot translation.
- Extensible plugin system.
- External HTTP API for integration with other applications.
- Wayland support.

---

## Step-by-step installation

Current builds are available in [GitHub Releases](https://github.com/frs777/pot-desktop/releases/tag/v0.1.1-0). See the [project website](https://frs777.github.io/pot-desktop/index-en.html) for the full description and links to every package.

### Fedora 43 (RPM)

1. Go to [Releases](https://github.com/frs777/pot-desktop/releases/tag/v0.1.1-0) and download:

   ```text
   Pot-F-0.1.1-0-1.x86_64.rpm
   ```

2. Install dependencies if they are not resolved automatically:

   ```bash
   sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
   ```

3. Install the application:

   ```bash
   sudo dnf install ./Pot-F-0.1.1-0-1.x86_64.rpm
   ```

4. Launch it from the application menu or terminal:

   ```bash
   pot-f
   ```

### Debian/Ubuntu (DEB)

1. Go to [Releases](https://github.com/frs777/pot-desktop/releases/tag/v0.1.1-0) and download:

   ```text
   Pot-F_0.1.1-0_amd64.deb
   ```

2. Install dependencies:

   ```bash
   sudo apt-get install -y libgtk-3-0 libwebkit2gtk-4.1-0 libayatana-appindicator3-1 librsvg2-2 patchelf xdotool libxcb1 libxrandr2 libdbus-1-3 tesseract-ocr
   ```

3. Install and launch Pot-F:

   ```bash
   sudo apt-get install ./Pot-F_0.1.1-0_amd64.deb
   pot-f
   ```

### Arch Linux / Manjaro

Install the `pot-f-desktop` package:

```bash
yay -S pot-f-desktop
# or
paru -S pot-f-desktop
```

---

## Dependencies

### Fedora 43

| Dependency | Purpose |
|---|---|
| gtk3-devel | GUI library |
| webkit2gtk4.1-devel | WebView engine |
| libayatana-appindicator-gtk3-devel | System tray icon |
| librsvg2-devel | SVG support |
| patchelf | ELF editing utility |
| xdotool-devel | Keyboard and mouse automation |
| libxcb-devel | X11 support |
| libXrandr-devel | Display resolution support |
| dbus-devel | Inter-process communication |
| tesseract | OCR engine |

Install in one command:

```bash
sudo dnf install gtk3-devel webkit2gtk4.1-devel libayatana-appindicator-gtk3-devel librsvg2-devel patchelf xdotool-devel libxcb-devel libXrandr-devel dbus-devel tesseract
```

### Debian/Ubuntu

| Dependency | Purpose |
|---|---|
| libgtk-3-0 | GUI library |
| libwebkit2gtk-4.1-0 | WebView engine |
| libayatana-appindicator3-1 | System tray icon |
| librsvg2-2 | SVG support |
| patchelf | ELF editing utility |
| xdotool | Keyboard and mouse automation |
| libxcb1 | X11 support |
| libxrandr2 | Display resolution support |
| libdbus-1-3 | Inter-process communication |
| tesseract-ocr | OCR engine |

Install in one command:

```bash
sudo apt-get install -y libgtk-3-0 libwebkit2gtk-4.1-0 libayatana-appindicator3-1 librsvg2-2 patchelf xdotool libxcb1 libxrandr2 libdbus-1-3 tesseract-ocr
```

---

## First launch

After the first launch, Pot-F appears in the system tray. By default:

1. **Interface language:** Polish.
2. **Translation services:** configure them in Preferences.
3. **Keyboard shortcuts:** set them in Preferences → Keyboard shortcuts.
4. **Configuration directory:** `~/.config/com.pot-f-desktop.desktop/`.

### Translation services

1. Open Preferences from the tray menu.
2. Open the translation services tab.
3. Add the services you want to use.
4. Enter your own API keys where required.
5. Save the settings.

### OCR services

1. Open Preferences.
2. Open the OCR services tab.
3. Tesseract is available locally on Linux when installed.
4. Add other OCR services if needed.

---

## Configuration

### Keyboard shortcuts

Shortcuts can be configured for:

- translating selected text,
- opening input translation,
- screenshot OCR,
- screenshot translation.

> **Wayland note:** application-level global shortcuts may not work under Wayland. Configure desktop-environment shortcuts that call the local HTTP API.

### Clipboard monitoring

Enable clipboard monitoring in the application or tray menu. Pot-F will translate copied text automatically.

### Integration with other applications

Pot-F exposes a local HTTP API on port `60828` by default:

```bash
# Translate selected text
curl "127.0.0.1:60828/selection_translate"

# Translate supplied text
curl -X POST -d "text to translate" "127.0.0.1:60828/translate"

# Open Preferences
curl "127.0.0.1:60828/config"
```

---

## Troubleshooting

### The application does not start

1. Verify that the required dependencies are installed.
2. Run `pot-f` in a terminal and inspect the error message.
3. On Windows, the original project may additionally require WebView2.

### Shortcuts do not work on Linux/Wayland

Configure a shortcut in your desktop environment and use, for example:

```bash
curl "127.0.0.1:60828/selection_translate"
```

### Tesseract OCR does not work

Check the installation:

```bash
tesseract --version
```

Install Polish language data:

```bash
# Fedora
sudo dnf install tesseract-langpack-pol

# Debian/Ubuntu
sudo apt-get install tesseract-ocr-pol
```

### Screenshots do not work on Wayland

Use an external screenshot utility.

Flameshot:

```bash
rm ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && flameshot gui -s -p ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

Grim and Slurp:

```bash
grim -g "$(slurp)" ~/.cache/com.pot-f-desktop.desktop/pot_screenshot_cut.png && curl "127.0.0.1:60828/ocr_recognize?screenshot=false"
```

### The tray icon does not appear

Install `libayatana-appindicator` or the equivalent package for your distribution. Some desktop environments require an additional tray extension.

### Translation services cannot connect

1. Check the Internet connection.
2. Verify API keys for services that require them.
3. Check whether a firewall blocks the connection.
4. Review the Pot-F log for the exact request error.

---

## Changes from the original project

### Version 0.1.0

- Renamed the application from `pot-desktop` to `pot-f-desktop`.
- Renamed the executable from `pot` to `pot-f`.
- Changed the identifier from `com.pot-app.desktop` to `com.pot-f-desktop.desktop`.
- Made Polish the default interface language.
- Disabled the original project's updater.
- Adapted RPM dependencies for Fedora.
- Updated links on the About page.
- Added fork information and Polish interface translations.
- Introduced the Pot-F version numbering.
- Added project documentation and repository metadata.

---

## Building from source

### Requirements

| Tool | Version | Purpose |
|---|---|---|
| Node.js | 18.0.0 or newer | JavaScript runtime |
| pnpm | 8.5.0 or newer | Package manager |
| Rust | 1.80.0 or newer | Compiler |
| Tauri CLI | Current 2.x release | Build tool |

### Install build tools on Fedora

```bash
sudo dnf install nodejs
npm install -g pnpm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install tauri-cli --version "^2"
```

### Install build tools on Debian/Ubuntu

```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
npm install -g pnpm
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install tauri-cli --version "^2"
```

### Build

```bash
git clone https://github.com/frs777/pot-desktop.git
cd pot-desktop
pnpm install
pnpm tauri dev
```

Build installation bundles:

```bash
pnpm tauri build
```

Generated bundles are placed under `src-tauri/target/release/bundle/`.

---

## Contact

- **GitHub:** https://github.com/frs777/pot-desktop
- **Issues:** https://github.com/frs777/pot-desktop/issues
- **Support email:** frss@protonmail.com
- **Discussions:** https://github.com/frs777/pot-desktop/discussions

---

## License

Pot-F Desktop is distributed under the GPL-3.0 license inherited from the original project.

Original project: [pot-app/pot-desktop](https://github.com/pot-app/pot-desktop) by [Pylogmon](https://github.com/pot-app).
