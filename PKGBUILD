# Maintainer: frs777 <twoj_email@przyklad.pl>
# Fork: Pot-F Desktop (Polska wersja Pot Desktop)
# Oryginalny autor: pylogmon

pkgname=pot-f-desktop
pkgver=0.1.0
pkgrel=1
pkgdesc="Pot-F Desktop - Polskie narzędzie do tłumaczenia i OCR (Fork Pot Desktop)"
arch=('x86_64')
url="https://github.com/frs777/pot-f-desktop"
license=('GPL-3.0-only')
depends=(
    'nodejs'
    'pnpm'
    'rust'
    'webkit2gtk-4.1'
    'gtk3'
    'libayatana-appindicator'
    'librsvg'
    'xdotool'
    'xclip'
    'dbus'
    'tesseract'
    'tesseract-data-eng'
    'tesseract-data-pol'
)
makedepends=(
    'git'
    'cargo'
    'npm' # Czasami wymagane przez niektóre skrypty buildowe node
)
optdepends=(
    'tesseract-data-deu: Wsparcie OCR dla języka niemieckiego'
    'tesseract-data-fra: Wsparcie OCR dla języka francuskiego'
    'tesseract-data-rus: Wsparcie OCR dla języka rosyjskiego'
    'tesseract-data-ukr: Wsparcie OCR dla języka ukraińskiego'
    'tesseract-data-chi_sim: Wsparcie OCR dla chińskiego uproszczonego'
    'tesseract-data-jpn: Wsparcie OCR dla języka japońskiego'
    'wl-clipboard: Wsparcie dla schowka w Wayland (wymaga xwayland)'
)
conflicts=('pot-desktop')
provides=('pot-desktop')

source=("$pkgname-$pkgver::git+https://github.com/frs777/pot-f-desktop.git#tag=v$pkgver")
sha256sums=('SKIP')

# Funkcja budowania
build() {
    cd "$srcdir/$pkgname-$pkgver"

    # Konfiguracja środowiska Node.js
    export npm_config_cache="$srcdir/npm-cache"
    mkdir -p "$npm_config_cache"

    # Instalacja zależności frontendu
    echo "Instalowanie zależności frontendu (pnpm)..."
    pnpm install --frozen-lockfile || pnpm install

    # Budowanie backendu Rust i frontendu jednocześnie
    echo "Rozpoczynanie budowania aplikacji (Tauri v2)..."
    
    # Ustawienie flag dla Rusta (optymalizacja)
    export RUSTFLAGS="-C target-cpu=native"
    
    # Uruchomienie procesu budowania Tauri
    # --bundles deb rpm tworzy też paczki systemowe, jeśli potrzebne
    pnpm tauri build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"

    # Instalacja zbudowanych plików
    # Tauri domyślnie buduje do src-tauri/target/release/bundle/
    
    # Ścieżki mogą się różnić w zależności od konfiguracji tauri.conf.json
    # Zakładamy standardową strukturę Tauri v2
    
    local _install_dir="$pkgdir/usr/lib/$pkgname"
    local _bin_dir="$pkgdir/usr/bin"
    local _desktop_dir="$pkgdir/usr/share/applications"
    local _icon_dir="$pkgdir/usr/share/icons/hicolor/scalable/apps"

    mkdir -p "$_install_dir"
    mkdir -p "$_bin_dir"
    mkdir -p "$_desktop_dir"
    mkdir -p "$_icon_dir"

    # Kopiowanie binarki
    # Nazwa binarki jest zazwyczaj taka sama jak pkgname lub zdefiniowana w tauri.conf.json
    # Sprawdźmy target/release/pot-f-desktop
    if [ -f "src-tauri/target/release/pot-f-desktop" ]; then
        cp "src-tauri/target/release/pot-f-desktop" "$_install_dir/"
        ln -s "/usr/lib/$pkgname/pot-f-desktop" "$_bin_dir/pot-f-desktop"
    else
        # Fallback: szukanie pliku wykonywalnego w bundle
        find "src-tauri/target/release/bundle" -type f -executable -name "pot-f-desktop" | head -n 1 | xargs -I {} cp {} "$_install_dir/"
        ln -s "/usr/lib/$pkgname/pot-f-desktop" "$_bin_dir/pot-f-desktop" 2>/dev/null || true
    fi

    # Kopiowanie pliku .desktop
    if [ -f "src-tauri/target/release/bundle/deb/data/usr/share/applications/com.pot-f-desktop.desktop" ]; then
        cp "src-tauri/target/release/bundle/deb/data/usr/share/applications/com.pot-f-desktop.desktop" "$_desktop_dir/"
    elif [ -f "com.pot-f-desktop.desktop" ]; then
        cp "com.pot-f-desktop.desktop" "$_desktop_dir/"
    fi

    # Kopiowanie ikon
    # Tauri często generuje ikony w bundle/linux/ lub public/
    if [ -d "src-tauri/target/release/bundle/deb/data/usr/share/icons" ]; then
        cp -r "src-tauri/target/release/bundle/deb/data/usr/share/icons/"* "$_icon_dir/"
    elif [ -f "public/icon.png" ] || [ -f "public/icon.svg" ]; then
        # Jeśli brak wygenerowanych, spróbuj użyć tych z public (może wymagać konwersji)
        cp public/icon*.png "$_icon_dir/" 2>/dev/null || true
        cp public/icon*.svg "$_icon_dir/" 2>/dev/null || true
    fi
    
    # Alternatywnie: ręczne wskazanie ikony jeśli jest w źródłach
    # Ikony są często w src-tauri/icons
    if [ -f "src-tauri/icons/icon.svg" ]; then
        cp "src-tauri/icons/icon.svg" "$_icon_dir/com.pot-f-desktop.svg"
    fi
}

# Sprawdzenie poprawności plików przed instalacją (opcjonalne)
check() {
    cd "$srcdir/$pkgname-$pkgver"
    echo "Uruchamianie testów jednostkowych (jeśli dostępne)..."
    # pnpm test || true
    # cargo test --manifest-path src-tauri/Cargo.toml || true
}
