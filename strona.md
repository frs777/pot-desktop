# Dokumentacja GitHub Pages - Projekt pot-desktop

## Podsumowanie stanu (stan na 2026-08-07 - OSTATECZNY SUKCES)

### Co działa (Status: OK):
- **Strona Główna (PL):** Dostępna bezpośrednio pod adresem: `https://frs777.github.io/pot-desktop/` (index.html).
- **Wersja Angielska (EN):** Dostępna pod adresem: `https://frs777.github.io/pot-desktop/index-en.html`.
- **Wdrożenie GitHub Pages:** Skonfigurowane z katalogu głównego (root `path: '.'`) za pomocą GitHub Actions (`deploy-pages.yml`). Całkowicie wyeliminowano błędy 404.
- **Odnośnik pobierania (Arch):** Przycisk pobierania paczki `.pkg.tar.zst` działa w 100% poprawnie. Prowadzi do fizycznego assetu w nowo utworzonym wydaniu **GitHub Release v0.1.0**.
- **Odnośnik "⭐ GitHub":** Prowadzi bezpośrednio do poprawnego repozytorium `https://github.com/frs777/pot-desktop`.
- **Przełączanie języków:** Działa poprawnie w obie strony.
- **Paczki w repozytorium:** Najnowsza paczka Arch (`pot-f-desktop-0.1.0-10-x86_64.pkg.tar.zst`) znajduje się w lokalnym katalogu `arch/` oraz w Release v0.1.0.

### Wyzwania, które rozwiązano:
- **Usunięcie błędnych przekierowań:** Wyeliminowano przekierowania przez nieistniejące foldery `docs/` i `www/`. Struktura została spłaszczona bezpośrednio do katalogu głównego.
- **GitHub Release:** Utworzono oficjalny release i pomyślnie wgrano do niego plik binarny paczki jako bezpieczny załącznik.
- **Naprawa literówek w linkach:** Poprawiono błędną ścieżkę repozytorium (`pot-f-desktop` -> `pot-desktop`).

### Co do zrobienia w przyszłości:
1. **Dodanie pozostałych paczek:** Po zbudowaniu paczek `.deb`, `.rpm` i `AppImage` należy dodać je do kolejnego Release'u i podlinkować w plikach HTML.
2. **Zachowanie spójności:** Wszystkie kolejne edycje strony głównej należy wprowadzać bezpośrednio w plikach `index.html` oraz `index-en.html` w katalogu głównym repozytorium.

## Notatki techniczne:
- Metoda wdrożenia: `GitHub Actions` (workflow `.github/workflows/deploy-pages.yml`).
- Repozytorium: `https://github.com/frs777/pot-desktop`
- Tag wydania: `v0.1.0`
