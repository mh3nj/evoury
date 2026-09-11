<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-først Kreativ Asset Manager</strong>
</p>

<p align="center">
  <a href="#features">Funktioner</a> •
  <a href="#installation">Installation</a> •
  <a href="#development">Udvikling</a> •
  <a href="#architecture">Arkitektur</a> •
  <a href="#contributing">Bidrag</a> •
  <a href="#license">Licens</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licens">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Om

Evoury er en kraftful, offline-først kreativ asset manager bygget med Tauri, React og Rust. Designet til kreative professionelle, der har brug for hurtig og pålidelig adgang til deres digitale aktiver uden at gå på kompromis med ydeevne eller privatliv.

### Hvorfor Evoury?

- **Offline-først**: Dine aktiver forbliver på din maskine. Ingen cloudafhængighed.
- **Blitzhurtig**: Bygget med Rust for ydeevne, der skalerer med dit bibliotek.
- **Modulær arkitektur**: 40+ specialiserede crates for maksimal fleksibilitet.
- **Smuk UI**: Moderat, responsivt interface bygget med React og Tailwind CSS.

---

## Funktioner

### Kerne motor

- **Multi-format support**: Billeder, videoer, 3D-modeller, lyd, dokumenter og mere
- **Smart pairing**: Grupperer automatisk relaterede filer
- **Asset statemachine**: Sporer aktiver fra opdagelse til arkivering
- **Hændelsesdrevet arkitektur**: Afkoblet kommunikation via hændelsesbus

### Bibliotekshåndtering

- **Avanceret scanner**: Komplet, inkrementel, mappe-specifik og baggrundsscanningstilstande
- **Filsystem watcher**: Realtidssynkronisering uden manuel opdatering
- **Metadata pipeline**: Automatisk ekstraktion, normalisering, validering og caching
- **Duplikatdetektion**: SHA256, perceptuel hashing og metadata-baseret

### Søgning og organisering

- **Permanent søgeindeks**: Lynhurtig fuldtekstsøgning med FTS5
- **Smart samlinger**: Regle-baserede automatisk opdaterende samlinger
- **Avanceret forespørgselssprog**: Filtre efter type, tag, bedømmelse, dato, kamera og mere
- **Søgeprofiler**: Gem og skift mellem søgekonfigurationer

### Workspace-system

- **Permanente workspaces**: Husker hele sessionstilstanden
- **Flere workspaces**: Skift mellem forskellige projekt kontekster
- **Workstations**: Forudkonfigurerede layouts med værktøjer, genveje og temaer
- **Dokkbare paneler**: Fuldt tilpasselig layout motor

### Sundhed og vedligeholdelse

- **Sundhedsmotor**: Tjekker integriteten af filsystem, database, cache og metadata
- **Automatisk reparation**: Et-klik reparation for opdagede problemer
- **Session gendannelse**: Gendanner workspace efter uventede nedlukninger
- **Sove tilstand**: Minimal ressourceforbrug i inaktivitet

---

## Skærmbilleder

<p align="center">
  <img src="public/images/main_dark.webp" alt="Hovedinterface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hovedinterface - Galleri visning</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspektør panel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspektør panel - Asset detaljer</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Søge interface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Avanceret søge interface</em>
</p>

---

## Installation

### Forudsætninger

- [Rust](https://www.rust-lang.org/tools/install) (seneste stabile version)
- [Node.js](https://nodejs.org/) (v18 eller nyere)
- [pnpm](https://pnpm.io/) (v8 eller nyere)

### Download

Download den seneste version fra [Releases](https://github.com/mh3nj/evoury/releases) siden.

### Byg fra kildekode

```bash
# Klon repoet
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installer afhængigheder
pnpm install

# Start udviklingsserveren
pnpm tauri dev

# Byg til produktion
pnpm tauri build
```

---

## Udvikling

### Tilgængelige kommandoer

```bash
# Udvikling
pnpm dev              # Start Vite udviklingsserver
pnpm tauri dev        # Start Tauri i udviklingstilstand

# Byg
pnpm build            # Byg frontend
pnpm tauri build      # Byg Tauri app til produktion

# Test
pnpm test             # Kør frontend tests
cargo test            # Kør Rust tests

# Lint
pnpm lint             # Kør ESLint
cargo clippy          # Kør Clippy

# Formatering
pnpm format           # Formater frontend kode
cargo fmt             # Formater Rust kode
```

---

## Tech Stack

### Backend

- **Rust** - Systemprogrammeringssprog
- **Tauri** - Desktop app framework
- **SQLite** - Lokal database
- **Crossbeam** - Konkurrenceprimitiver

### Frontend

- **React** - UI bibliotek
- **TypeScript** - Typesikkert JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Tilstandshåndtering
- **Vite** - Byggeværktøj og udviklingsserver

---

## Køreplan

Se [ROADMAP.md](ROADMAP.md) for detaljeret udviklingskøreplan.

---

## Bidrag

Bidrag er velkomne! Læs venligst [CONTRIBUTING.md](CONTRIBUTING.md) først.

---

## Licens

Dette projekt er licenseret under MIT-licensen - se filen [LICENSE](LICENSE) for detaljer.

---

## Support

- **Problemer**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskussioner**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Lavet med ❤️ af <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
