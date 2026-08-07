# Dokumentacja GitHub Pages - Projekt pot-desktop

## Podsumowanie stanu (stan na 2026-08-07)

### Co działa (Status: OK):
- **Repozytorium:** Synchronizacja z GitHub (`frs777/pot-desktop`) przebiega poprawnie.
- **Paczki:** Najnowsza paczka Arch (`pot-f-desktop-0.1.0-10-x86_64.pkg.tar.zst`) została poprawnie dodana do repozytorium lokalnego (`moje-repo`) i jest podlinkowana na stronie pobierania.
- **Odnośniki:** Przycisk "Download Arch" poprawnie prowadzi do paczki w repozytorium (format `?raw=true`). Przycisk "English" poprawnie przełącza stronę.
- **GitHub Actions:** Workflow `deploy-pages.yml` kończy się sukcesem, wdrażając stronę.

### Co nie działa / Wyzwania:
- **Błąd 404:** Mimo poprawnych wdrożeń (zielone Actions), główna strona (lub przekierowania) często zwraca 404. Problemem jest konfiguracja `build_type: workflow` w GitHub Pages i mapowanie folderów.
- **Struktura katalogów:** Próby przenoszenia plików między `docs/`, `www/`, a `root/` powodowały konflikty w GitHub Pages.
- **Ustawienia GitHub:** Interfejs GitHub Pages w ustawieniach repozytorium jest niejasny i czasami ukrywa opcję "GitHub Actions" dla wdrożeń.

### Co jest do zrobienia:
1. **Stabilizacja struktury:** Jeśli obecne rozwiązanie (wdrażanie całego root) nadal będzie zwracać 404, należy rozważyć finalne przeniesienie wszystkich plików strony głównej (`index.html`, `index-en.html`, `README.md`) na stałe do katalogu głównego (root) i usunięcie podfolderów typu `docs/` czy `www/`.
2. **Dodanie pozostałych paczek:** Po zbudowaniu paczek `.deb`, `.rpm` i `AppImage`, należy je również umieścić w repozytorium i podlinkować na stronie w sekcji pobierania.
3. **Testy:** Po każdej zmianie w strukturze plików, należy odczekać 2-3 minuty na odświeżenie cache przez GitHub Pages i przetestować stronę w trybie incognito.

## Notatki techniczne:
- Wdrożenie: `GitHub Actions` (workflow `.github/workflows/deploy-pages.yml`).
- Główny adres strony: `https://frs777.github.io/pot-desktop/`.
- Repozytorium paczek lokalnych: `/home/frs/RepoArch/x86_64/`.

## Aktualizacja dokumentacji (2026-08-07)
- Pliki stron (PL/EN) zostały uporządkowane w katalogu docs/.
- W katalogu głównym znajduje się tylko plik index.html z przekierowaniem do /docs/.
- Workflow GitHub Actions wdraża teraz katalog docs/.
- Przycisk pobierania paczki Arch jest naprawiony.
## Aktualizacja dokumentacji (2026-08-07 - Ostateczna)
- Przejście na płaską strukturę w root w celu naprawy 404.
- Usunięto katalog 'docs/'.
- Pliki stron są teraz bezpośrednio w głównym katalogu.