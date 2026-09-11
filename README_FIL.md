<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-Muna Tagapamahala ng Mapagkakahalong Assets</strong>
</p>

<p align="center">
  <a href="#features">Mga Tampok</a> •
  <a href="#installation">Pag-install</a> •
  <a href="#development">Pag-unlad</a> •
  <a href="#architecture">Arkitektura</a> •
  <a href="#contributing">Pag-aambag</a> •
  <a href="#license">Lisensya</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Bersyon">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lisensya">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Tungkol sa

Ang Evoury ay isang makapangyarihang, offline-muna tagapamahala ng mapagkakahalong assets na ginawa gamit ang Tauri, React at Rust. Dinisenyo para sa mga propesyonal na may malikhaing pangangailangan para sa mabilis at maaasahang access sa kanilang mga digital assets nang walang kompromiso sa pribasiya o pagganap.

### Bakit Evoury?

- **Offline-muna**: Ang iyong mga assets ay nananatili sa iyong makina. Walang cloud dependency.
- **Mabilis na parang kidlat**: Ginawa gamit ang Rust para sa pagganap na naaayon sa iyong aklatan.
- **Modular na arkitektura**: Higit sa 40 espesyal na crate para sa maximum na kakayahang umangkop.
- **Magandang UI**: Modernong, tumutugon na interface na ginawa gamit ang React at Tailwind CSS.

---

## Mga Tampok

### Pangunahing Makina

- **Suporta sa multi-format**: Mga larawan, bidyo, 3D model, audio, dokumento at marami pa
- **Matalinong pagpapares**: Awtomatikong nag-grupo ng magkakaugnay na file
- **Asset state machine**: Sinusubaybayan ang mga assets mula sa pagtuklas hanggang sa pag-arkiba
- **Event-driven na arkitektura**: Nakahiwalay na komunikasyon sa pamamagitan ng event bus

### Pamamahala ng Aklatan

- **Advanced na scanner**: Buong scan, incremental, folder-specific at background na mga mode ng pag-scan
- **Filesystem watcher**: Real-time na pag-sync nang walang manual na pag-reload
- **Metadata pipeline**: Awtomatikong pagkuha, pag-normilize, pag-validate at pag-cache
- **Pag-detect ng duplicate**: SHA256, perceptual hashing at metadata-based

### Paghahanap at Organisasyon

- **Persistent na search index**: Mabilis na full-text search gamit ang FTS5
- **Matalinong koleksyon**: Rule-based na auto-updating na mga koleksyon
- **Advanced na query language**: Mga filter ayon sa uri, tag, rating, petsa, camera at marami pa
- **Search profiles**: I-save at mag-switch sa pagitan ng mga search configuration

### Workspace System

- **Persistent na workspace**: Naalala ang buong estado ng sesyon
- **Maramihang workspace**: Mag-switch sa pagitan ng iba't ibang konteksto ng proyekto
- **Workstation**: Mga pre-configured na layout na may mga kasangkapan, shortcut at tema
- **Dockable panel**: ganap na naa-customize na layout engine

### Kalusugan at Pagpapanatili

- **Health engine**: Sinusuri ang integridad ng filesystem, database, cache at metadata
- **Awtomatikong pag-aayos**: Isang-klik na pag-aayos para sa mga nakitang problema
- **Session recovery**: Binabalik ang workspace pagkatapos ng hindi inaasahang pag-shutdown
- **Sleep mode**: Minimum na paggamit ng resources kapag walang ginagawa

---

## Mga Screenshot

<p align="center">
  <img src="public/images/main_dark.webp" alt="Pangunahing Interface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Pangunahing Interface - Gallery View</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspector Panel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspector Panel - Mga Detalye ng Asset</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Search Interface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Advanced na Search Interface</em>
</p>

---

## Pag-install

### Mga Kinakailangan

- [Rust](https://www.rust-lang.org/tools/install) (pinakabagong stable na bersyon)
- [Node.js](https://nodejs.org/) (v18 o mas bago)
- [pnpm](https://pnpm.io/) (v8 o mas bago)

### Pag-download

I-download ang pinakabagong bersyon mula sa [Releases](https://github.com/mh3nj/evoury/releases) page.

### Pag-build mula sa Source

```bash
# I-clone ang repository
git clone https://github.com/mh3nj/evoury.git
cd evoury

# I-install ang mga dependencies
pnpm install

# Simulan ang development server
pnpm tauri dev

# I-build para sa production
pnpm tauri build
```

---

## Pag-unlad

### Mga Available na Command

```bash
# Pag-unlad
pnpm dev              # Simulan ang Vite dev server
pnpm tauri dev        # Simulan ang Tauri sa development mode

# Pag-build
pnpm build            # I-build ang frontend
pnpm tauri build      # I-build ang Tauri app para sa production

# Mga Pagsubok
pnpm test            # Patakbuhin ang mga frontend test
cargo test           # Patakbuhin ang mga Rust test

# Lint
pnpm lint            # Patakbuhin ang ESLint
cargo clippy         # Patakbuhin ang Clippy

# Pag-format
pnpm format          # I-format ang frontend code
cargo fmt            # I-format ang Rust code
```

---

## Tech Stack

### Backend

- **Rust** - Wika ng sistema ng program
- **Tauri** - Framework ng desktop app
- **SQLite** - Lokal na database
- **Crossbeam** - Mga primitive para sa concurrency

### Frontend

- **React** - UI library
- **TypeScript** - Type-safe na JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Pamamahala ng estado
- **Vite** - Build tool at dev server

---

## Roadmap

Tingnan ang [ROADMAP.md](ROADMAP.md) para sa detalyadong roadmap ng pag-unlad.

---

## Pag-aambag

Malugod na tinatanggap ang mga ambag! Mangyaring basahin muna ang [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Lisensya

Ang proyektong ito ay lisensyado sa ilalim ng MIT Lisensya - tingnan ang file na [LICENSE](LICENSE) para sa mga detalye.

---

## Suporta

- **Mga Isyu**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Mga Talakayan**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Ginawa ng ❤️ ni <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
