<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-first Kreativ Tillgångshanterare</strong>
</p>

<p align="center">
  <a href="#features">Funktioner</a> •
  <a href="#installation">Installation</a> •
  <a href="#development">Utveckling</a> •
  <a href="#architecture">Arkitektur</a> •
  <a href="#contributing">Bidra</a> •
  <a href="#license">Licens</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licens">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plattform">
</p>

---

## Om

Evoury är en kraftfull, offline-first kreativ tillgångshanterare byggd med Tauri, React och Rust. Designad för kreativa professionella som behöver snabb och tillförlitlig åtkomst till sina digitala tillgångar utan kompromisser prestanda eller integritet.

### Varför Evoury?

- **Offline-first**: Dina tillgångar förbliver på din dator. Inget molnberoende.
- **Blixtnabb**: Byggd med Rust för prestanda som skalar med ditt bibliotek.
- **Modulär arkitektur**: 40+ specialiserade cratess för maximal flexibilitet.
- **Vacker UI**: Modern, responsiv gränsbyta byggd med React och Tailwind CSS.

---

## Funktioner

### Kärnmotor

- **Multi-formatstöd**: Bilder, videor, 3D-modeller, ljud, dokument och mer
- **Smart parning**: Grupperar automatiskt relaterade filer
- **Tillståndsmaskin**: Spårar tillgångar från upptäckt till arkivering
- **Händelsedriven arkitektur**: Avkopplad kommunikation via händelsebuss

### Bibliotekshantering

- **Avancerad skanner**: Fullständig, inkrementell, mapp-specifik och bakgrundsläge
- **Filsystemövervakare**: Realtidssynkronisering utan manuell uppdatering
- **Metadata-rörledning**: Automatisk extrahering, normalisering, validering och caching
- **Dubblettidentifiering**: SHA256, uppfattningsbaserad hashing och metadata-baserad

### Sökning och Organisation

- **Permanent sökindex**: Blixtnabb fulltextsökning med FTS5
- **Smarta samlingar**: Regelbaserade automatiskt uppdaterade samlingar
- **Avancerat frågespråk**: Filtrering efter typ, tagg, betyg, datum, kamera och mer
- **Sökningsprofiler**: Spara och växla mellan sökkonfigurationer

### Arbetsyta-system

- **Permanenta arbetsytor**: Kommer ihåg hela sessionstillståndet
- **Flera arbetsytor**: Växla mellan olika projektkontexter
- **Arbetsstationer**: Förkonfigurerade layouter med verktyg, genvägar och teman
- **Dockningsbara paneler**: Helt anpassningsbar layoutmotor

### Hälsa och Underhåll

- **Hälsomotor**: Kontrollerar integriteten för filsystem, databas, cache och metadata
- **Automatisk reparation**: En klicks reparation för upptäckta problem
- **Sessionåterställning**: Återställer arbetsutan efter oväntade avstängningar
- **Viloläge**: Minimal resursanvändning vid overksamhet

---

## Skärmbilder

<p align="center">
  <img src="public/images/main_dark.webp" alt="Huvudgränssnitt" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Huvudgränssnitt - Gallerivisning</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspektörspanel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspektörspanel - Tillgångsdetaljer</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Sökningsgränssnitt" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Avancerat sökningsgränssnitt</em>
</p>

---

## Installation

### Förutsättningar

- [Rust](https://www.rust-lang.org/tools/install) (senaste stabila version)
- [Node.js](https://nodejs.org/) (v18 eller högre)
- [pnpm](https://pnpm.io/) (v8 eller högre)

### Hämtning

Hämta den senaste versionen från [Releases](https://github.com/mh3nj/evoury/releases) sidan.

### Bygg från källkod

```bash
# Klona repot
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installera beroenden
pnpm install

# Starta utvecklingsservern
pnpm tauri dev

# Bygg för produktion
pnpm tauri build
```

---

## Utveckling

### Tillgängliga kommandon

```bash
# Utveckling
pnpm dev              # Starta Vite-utvecklingsservern
pnpm tauri dev        # Starta Tauri i utvecklingsläge

# Bygg
pnpm build            # Bygg frontend
pnpm tauri build      # Bygg Tauri-appen för produktion

# Tester
pnpm test             # Kör frontend-tester
cargo test            # Kör Rust-tester

# Lint
pnpm lint             # Kör ESLint
cargo clippy          # Kör Clippy

# Formatering
pnpm format           # Formatera frontend-kod
cargo fmt             # Formatera Rust-kod
```

---

## Teknisk stapel

### Backend

- **Rust** - Systemprogrammeringsspråk
- **Tauri** - Skrivbordsapplikationsramverk
- **SQLite** - Lokal databas
- **Crossbeam** - Konkurrensprimitiver

### Frontend

- **React** - UI-bibliotek
- **TypeScript** - Typsäker JavaScript
- **Tailwind CSS** - Verktygsbaserat CSS-ramverk
- **Zustand** - Tillståndshanterare
- **Vite** - Byggverktyg och utvecklingsserver

---

## Vägkarta

Se [ROADMAP.md](ROADMAP.md) för detaljerad utvecklingsvägkarta.

---

## Bidra

Bidrag är välkomna! Vänligen läs [CONTRIBUTING.md](CONTRIBUTING.md) först.

---

## Licens

Det här projektet är licensierat under MIT-licensen - se filen [LICENSE](LICENSE) för detaljer.

---

## Stöd

- **Problem**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskussioner**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Tillverkad med ❤️ av <a href="https://github.com/mh3nj">Ditt namn</a>
</p>
