<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-Prvý Kreatívny Manažér Aktív</strong>
</p>

<p align="center">
  <a href="#features">Funkcie</a> •
  <a href="#installation">Inštalácia</a> •
  <a href="#development">Vývoj</a> •
  <a href="#architecture">Architektúra</a> •
  <a href="#contributing">Prispievanie</a> •
  <a href="#license">Licencia</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Verzia">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licencia">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforma">
</p>

---

## O Projekte

Evoury je výkonný, offline-prvý kreatívny manažér aktív postavený pomocou Tauri, React a Rust. Navrhnutý pre kreatívnych profesionálov, ktorí potrebujú rýchly a spoľahlivý prístup k svojim digitálnym aktívam bez kompromisov vo výkone alebo súkromí.

### Prečo Evoury?

- **Offline-prvý**: Vaše aktíva zostanú na vašom počítači. Žiadna cloudová závislosť.
- **Bleskurýchly**: Postavený pomocou Rust pre výkon, ktorý škáluje s vašou knižnicou.
- **Modulárna architektúra**: Viac ako 40 špecializovaných crate-ov pre maximálnu flexibilitu.
- **Krásne UI**: Moderné, responzívne rozhranie postavené pomocou React a Tailwind CSS.

---

## Funkcie

### Hlavný Motor

- **Multi-formátová podpora**: Obrázky, videá, 3D modely, zvuk, dokumenty a viac
- **Inteligentné párovanie**: Automaticky zoskupuje súvisiace súbory
- **Stroj stavov aktív**: Sleduje aktíva od objavenia po archiváciu
- **Udalosťami riadená architektúra**: Oddelená komunikácia cez zbernú udalostí

### Správa Knížníc

- **Pokročilý skener**: Plné, prírastkové, priečinkovo-špecifické a pozadové režimy skenovania
- **Pozorovateľ súborového systému**: Synchronizácia v reálnom čase bez ručneho obnovenia
- **Metadátový pipeline**: Automatická extrakcia, normalizácia, validácia a cacheovanie
- **Detekcia duplicitných**: SHA256, percepčné hašovanie a metadátové

### Vyhľadávanie a Organizácia

- **Perzistentný vyhľadávací index**: Bleskurýchle plnotextové vyhľadávanie s FTS5
- **Inteligentné kolekcie**: Pravidlami založené automaticky aktualizované kolekcie
- **Pokročilý jazyk dotazov**: Filtre podľa typu, značky, hodnotenia, dátumu, fotoaparátu a viac
- **Vyhľadávacie profily**: Uložte a prepínajte medzi konfiguráciami vyhľadávania

### Systém Pracovného Priestoru

- **Perzistentné pracovné priestory**: Pamätá celý stav relácie
- **Viac pracovných priestorov**: Prepínanie medzi rôznymi kontextami projektov
- **Pracovné stanice**: Predkonfigurované rozloženia s nástrojmi, skratkami a motívmi
- **Dokovateľné panely**: Plne prispôsobiteľný motor rozloženia

### Zdravie a Údržba

- **Zdravotný motor**: Kontroluje integritu súborového systému, databázy, cache a metadát
- **Automatická oprava**: Oprava jedným kliknutím pre detekované problémy
- **Obnovenie relácie**: Obnoví pracovný priestor po neočakávaných vypnutíach
- **Režim spánku**: Minimálne využitie zdrojov v nečinnosti

---

## Snímky Obrazovky

<p align="center">
  <img src="public/images/main_dark.webp" alt="Hlavné Rozhranie" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hlavné Rozhranie - Zobrazenie Galérie</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Panel Inspektora" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel Inspektora - Podrobnosti Aktíva</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Vyhľadávacie Rozhranie" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Pokročilé Vyhľadávacie Rozhranie</em>
</p>

---

## Inštalácia

### Predpoklady

- [Rust](https://www.rust-lang.org/tools/install) (najnovšia stabilná verzia)
- [Node.js](https://nodejs.org/) (v18 alebo novší)
- [pnpm](https://pnpm.io/) (v8 alebo novší)

### Sťahovanie

Stiahnite si najnovšiu verziu zo stránky [Releases](https://github.com/mh3nj/evoury/releases).

### Kompilácia zo Zdrojového Kódu

```bash
# Klonujte repozitár
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Inštalujte závislosti
pnpm install

# Spustite vývojový server
pnpm tauri dev

# Kompilujte pre produkciu
pnpm tauri build
```

---

## Vývoj

### Dostupné Príkazy

```bash
# Vývoj
pnpm dev              # Spustite Vite vývojový server
pnpm tauri dev        # Spustite Tauri vo vývojovom režime

# Kompilácia
pnpm build            # Kompilujte frontend
pnpm tauri build      # Kompilujte Tauri aplikáciu pre produkciu

# Testy
pnpm test             # Spustite frontend testy
cargo test            # Spustite Rust testy

# Lint
pnpm lint             # Spustite ESLint
cargo clippy          # Spustite Clippy

# Formátovanie
pnpm format           # Formátujte frontend kód
cargo fmt             # Formátujte Rust kód
```

---

## Tech Stack

### Backend

- **Rust** - Systémový programovací jazyk
- **Tauri** - Framework desktopových aplikácií
- **SQLite** - Lokálna databáza
- **Crossbeam** - Primitívy pre súbežnosť

### Frontend

- **React** - UI knižnica
- **TypeScript** - Typovo bezpečný JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Správa stavu
- **Vite** - Build nástroj a vývojový server

---

## Mapa Cesty

Podrobnú mapu vývoja nájdete v [ROADMAP.md](ROADMAP.md).

---

## Prispievanie

Príspevky sú vítané! Prečítajte si prosím najprv [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licencia

Tento projekt je licencovaný pod MIT licenciou - podrobnosti viz súbor [LICENSE](LICENSE).

---

## Podpora

- **Problémy**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskusie**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Vytvorené s ❤️ od <a href="https://github.com/mh3nj">Vaše meno</a>
</p>
