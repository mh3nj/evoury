<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-First Menedżer Zasobów Kreatywnych</strong>
</p>

<p align="center">
  <a href="#features">Funkcje</a> •
  <a href="#installation">Instalacja</a> •
  <a href="#development">Rozwój</a> •
  <a href="#architecture">Architektura</a> •
  <a href="#contributing">Wkład</a> •
  <a href="#license">Licencja</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Wersja">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licencja">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforma">
</p>

---

## O Projekcie

Evoury to potężny, offline-first menedżer zasobów kreatywnych, zbudowany za pomocą Tauri, React i Rust. Zaprojektowany dla profesjonalistów kreatywnych, którzy potrzebują szybkiego i niezawodnego dostępu do swoich zasobów cyfrowych bez kompromisów w wydajności i prywatności.

### Dlaczego Evoury?

- **Offline-first**: Twoje zasoby pozostają na Twoim komputerze. Bez zależności od chmury.
- **Błyskawiczna wydajność**: Zbudowany z Rust dla wydajności skalowanej z biblioteką.
- **Modularna architektura**: Ponad 40 specjalistycznych crate'ów dla maksymalnej elastyczności.
- **Piękny interfejs**: Nowoczesny, responsywny interfejs zbudowany z React i Tailwind CSS.

---

## Funkcje

### Główny Silnik

- **Wieloformatowe wsparcie**: Obrazy, filmy, modele 3D, audio, dokumenty i więcej
- **Inteligentne parowanie**: Automatycznie grupuje powiązane pliki
- **Maszyna stanów**: Śledzi zasoby od odkrycia do archiwizacji
- **Architektura zdarzeniowa**: Odseparowana komunikacja przez magistralę zdarzeń

### Zarządzanie Bibliotekami

- **Zaawansowany skaner**: Tryby skanowania pełnego, przyrostowego, folderowego i w tle
- **Obserwator systemu plików**: Synchronizacja w czasie rzeczywistym bez ręcznego odświeżania
- **Pipeline metadanych**: Automatyczne wyodrębnianie, normalizacja, walidacja i buforowanie
- **Wykrywanie duplikatów**: SHA256, hash percepcyjny i oparty na metadanych

### Wyszukiwanie i Organizacja

- **Trwały indeks wyszukiwania**: Błyskawiczne wyszukiwanie pełnotekstowe z FTS5
- **Inteligentne kolekcje**: Kolekcje oparte na regułach z automatyczną aktualizacją
- **Zaawansowany język zapytań**: Filtry wg typu, tagu, oceny, daty, aparatu i więcej
- **Profile wyszukiwania**: Zapisuj i przełączaj między konfiguracjami wyszukiwania

### System Przestrzeni Roboczej

- **Trwałe przestrzenie robocze**: Zapamiętuje cały stan sesji
- **Wiele przestrzeni**: Przełączanie między kontekstami projektów
- **Stacje robocze**: Wstępnie skonfigurowane układy z narzędziami, skrótami i motywami
- **Panel dockable**: W pełni konfigurowalny silnik układów

### Zdrowie i Konserwacja

- **Silnik zdrowia**: Sprawdza integralność systemu plików, bazy danych, bufora i metadanych
- **Automatyczna naprawa**: Naprawa jednym kliknięciem dla wykrytych problemów
- **Odzyskiwanie sesji**: Przywraca przestrzeń roboczą po nieoczekiwanych zamknięciach
- **Tryb uśpienia**: Minimalne zużycie zasobów w bezczynności

---

## Zrzuty Ekranu

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Główny interfejs" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Główny interfejs - Widok galerii</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Panel inspektora" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel inspektora - Szczegóły zasobu</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Interfejs wyszukiwania" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Zaawansowany interfejs wyszukiwania</em>
</p>

---

## Instalacja

### Wymagania Wstępne

- [Rust](https://www.rust-lang.org/tools/install) (najnowsza stabilna wersja)
- [Node.js](https://nodejs.org/) (v18 lub nowszy)
- [pnpm](https://pnpm.io/) (v8 lub nowszy)

### Pobieranie

Pobierz najnowszą wersję ze strony [Releases](https://github.com/mh3nj/evoury/releases).

### Budowanie ze Źródeł

```bash
# Klonuj repozytorium
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Zainstaluj zależności
pnpm install

# Uruchom serwer deweloperski
pnpm tauri dev

# Zbuduj dla produkcji
pnpm tauri build
```

---

## Rozwój

### Dostępne Polecenia

```bash
# Rozwój
pnpm dev              # Uruchom serwer Vite
pnpm tauri dev        # Uruchom Tauri w trybie deweloperskim

# Budowanie
pnpm build            # Zbuduj frontend
pnpm tauri build      # Zbuduj aplikację Tauri

# Testy
pnpm test             # Uruchom testy frontend
cargo test            # Uruchom testy Rust

# Lint
pnpm lint             # Uruchom ESLint
cargo clippy          # Uruchom Clippy

# Formatowanie
pnpm format           # Formatuj kod frontend
cargo fmt             # Formatuj kod Rust
```

---

## Stack Technologiczny

### Backend

- **Rust** - Język programowania systemowego
- **Tauri** - Framework aplikacji desktopowych
- **SQLite** - Baza danych lokalna
- **Crossbeam** - Primitywy programowania współbieżnego

### Frontend

- **React** - Biblioteka UI
- **TypeScript** - Bezpieczny typowo JavaScript
- **Tailwind CSS** - Framework CSS utility-first
- **Zustand** - Zarządzanie stanem
- **Vite** - Narzędzie budowania i serwer deweloperski

---

## Roadmapa

Zobacz [ROADMAP.md](ROADMAP.md) dla szczegółowej roadmapy rozwoju.

---

## Wkład

Wkłady są mile widziane! Przeczytaj najpierw [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licencja

Ten projekt jest licencjonowany na licencji MIT - zobacz plik [LICENSE](LICENSE) po szczegóły.

---

## Wsparcie

- **Problemy**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Dyskusje**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Zrobione z ❤️ przez <a href="https://github.com/mh3nj">Twoja Nazwa</a>
</p>
