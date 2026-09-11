<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-first Creatief Assetbeheer</strong>
</p>

<p align="center">
  <a href="#features">Functies</a> •
  <a href="#installation">Installatie</a> •
  <a href="#development">Ontwikkeling</a> •
  <a href="#architecture">Architectuur</a> •
  <a href="#contributing">Bijdragen</a> •
  <a href="#license">Licentie</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versie">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licentie">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Over

Evoury is een krachtige, offline-first creatief assetbeheer gebouwd met Tauri, React en Rust. Ontworpen voor creatieve professionals die snelle, betrouwbare toegang tot hun digitale assets nodig hebben zonder concessies te doen aan prestaties of privacy.

### Waarom Evoury?

- **Offline-first**: Je assets blijven op je machine. Geen cloudafhankelijkheid.
- **Bliksemsnel**: Gebouwd met Rust voor prestaties die schalen met je bibliotheek.
- **Modulaire architectuur**: 40+ gespecialiseerde crates voor maximale flexibiliteit.
- **Mooie UI**: Moderne, responsieve interface gebouwd met React en Tailwind CSS.

---

## Functies

### Kernmotor

- **Multi-formaat ondersteuning**: Afbeeldingen, video's, 3D-modellen, audio, documenten en meer
- **Slimme koppeling**: Groepeert automatisch gerelateerde bestanden
- **Asset-state-machine**: Volgt assets van ontdekking tot archivering
- **Gebeurtenis-gestuurde architectuur**: Ontkoppelde communicatie via eventbus

### Bibliotheekbeheer

- **Geavanceerde scanner**: Volledige, incrementele, map-specifieke en achtergrond scanmodi
- **Bestandssysteem watcher**: Real-time synchronisatie zonder handmatige vernieuwing
- **Metadata-pipeline**: Automatische extractie, normalisatie, validatie en caching
- **Dubbele detectie**: SHA256, perceptueel hashen en metadata-gebaseerd

### Zoeken en Organiseren

- **Permanente zoekindex**: Bliksemsnelle full-text zoekopdrachten met FTS5
- **Slimme collecties**: Regel-gebaseerde automatisch bijwerkende collecties
- **Geavanceerde querytaal**: Filters op type, tag, beoordeling, datum, camera en meer
- **Zoekprofielen**: Snel wisselen tussen zoekconfiguraties

### Workspace-systeem

- **Permanente workspaces**: Onthoudt de volledige sessiestatus
- **Meerdere workspaces**: Wissel tussen verschillende projectcontexten
- **Werkstations**: Voorgeconfigureerde lay-outs met gereedschappen, sneltoetsen en thema's
- **Dockable panelen**: Volledig aanpasbaar lay-outsysteem

### Gezondheid en Onderhoud

- **Gezondheidsmotor**: Controleert integriteit van bestandssysteem, database, cache en metadata
- **Automatische reparatie**: Eén-klik reparatie voor gedetecteerde problemen
- **Sessieherstel**: Herstelt workspace na onverwachte uitschakelingen
- **Slaapmodus**: Minimaal resourcegebruik in rust

---

## Screenshots

<p align="center">
  <img src="public/images/main_dark.webp" alt="Hoofdinterface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hoofdinterface - Galerijweergave</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspecteurpaneel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspecteurpaneel - Assetdetails</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Zoekinterface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Geavanceerde zoekinterface</em>
</p>

---

## Installatie

### Vereisten

- [Rust](https://www.rust-lang.org/tools/install) (nieuwste stabiele versie)
- [Node.js](https://nodejs.org/) (v18 of hoger)
- [pnpm](https://pnpm.io/) (v8 of hoger)

### Download

Download de nieuwste versie van de [Releases](https://github.com/mh3nj/evoury/releases) pagina.

### Bouwen vanuit Broncode

```bash
# Clone de repository
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installeer afhankelijkheden
pnpm install

# Start de ontwikkelingsserver
pnpm tauri dev

# Bouw voor productie
pnpm tauri build
```

---

## Ontwikkeling

### Beschikbare Commando's

```bash
# Ontwikkeling
pnpm dev              # Start Vite ontwikkelingsserver
pnpm tauri dev        # Start Tauri in ontwikkelingsmodus

# Bouwen
pnpm build            # Bouw frontend
pnpm tauri build      # Bouw Tauri-app voor productie

# Tests
pnpm test             # Voer frontend tests uit
cargo test            # Voer Rust tests uit

# Lint
pnpm lint             # Voer ESLint uit
cargo clippy          # Voer Clippy uit

# Formattering
pnpm format           # Formatteer frontend code
cargo fmt             # Formatteer Rust code
```

---

## Tech Stack

### Backend

- **Rust** - Systeemprogrammeertaal
- **Tauri** - Desktop-app framework
- **SQLite** - Lokale database
- **Crossbeam** - Concurrency primitives

### Frontend

- **React** - UI-bibliotheek
- **TypeScript** - Type-veilig JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Statusbeheer
- **Vite** - Build-tool en ontwikkelingsserver

---

## Roadmap

Zie [ROADMAP.md](ROADMAP.md) voor de gedetailleerde ontwikkelingsroadmap.

---

## Bijdragen

Bijdragen zijn welkom! Lees eerst [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licentie

Dit project is gelicentieerd onder de MIT-licentie - zie het bestand [LICENSE](LICENSE) voor details.

---

## Ondersteuning

- **Problemen**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discussies**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Gemaakt met ❤️ door <a href="https://github.com/mh3nj">Jouw Naam</a>
</p>
