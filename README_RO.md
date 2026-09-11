<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-first Manager de Active Creative</strong>
</p>

<p align="center">
  <a href="#features">Funcționalități</a> •
  <a href="#installation">Instalare</a> •
  <a href="#development">Dezvoltare</a> •
  <a href="#architecture">Arhitectură</a> •
  <a href="#contributing">Contribuire</a> •
  <a href="#license">Licență</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versiune">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licență">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platformă">
</p>

---

## Despre

Evoury este un manager de active creative offline-first, construit cu Tauri, React și Rust. Conceput pentru profesioniștii creativi care au nevoie de acces rapid și fiabil la activele lor digitale fără a compromite performanța sau confidențialitatea.

### De ce Evoury?

- **Offline-first**: Activele tale rămân pe mașina ta. Nicio dependență de cloud.
- **Ultra rapid**: Construit cu Rust pentru performanță care se scalează cu biblioteca ta.
- **Arhitectură modulară**: Peste 40 de crate-uri specializate pentru maximă flexibilitate.
- **Interfață frumoasă**: Interfață modernă, receptivă construită cu React și Tailwind CSS.

---

## Funcționalități

### Motor Principal

- **Suport multi-format**: Imagini, videoclipuri, modele 3D, audio, documente și multe altele
- **Împerechere inteligentă**: Grupează automat fișierele corelate
- **Mașina de stări**: Urmărește activele de la descoperire la arhivare
- **Arhitectură bazată pe evenimente**: Comunicare decuplată prin bus de evenimente

### Gestionare Bibliotecă

- **Scanner avansat**: Moduri de scanare completă, incrementală, specifică folderului și în fundal
- **Observator sistem de fișiere**: Sincronizare în timp real fără reîmprospătare manuală
- **Pipeline metadate**: Extragere, normalizare, validare și cache automat
- **Detectare duplicate**: SHA256, hash perceptual și bazat pe metadate

### Căutare și Organizare

- **Index de căutare persistent**: Căutare full-text ultra-rapidă cu FTS5
- **Colecții inteligente**: Colecții bazate pe reguli cu actualizare automată
- **Limba de interogare avansată**: Filtre după tip, etichetă, evaluare, dată, cameră și multe altele
- **Profile de căutare**: Salvează și comută între configurațiile de căutare

### Sistem Workspace

- **Workspace-uri persistente**: Își amintește întreaga stare a sesiunii
- **Workspace-uri multiple**: Comută între diferite contexte de proiect
- **Stații de lucru**: Layout-uri preconfigurate cu instrumente, scurtături și teme
- **Panouri andocabile**: Motor de layout complet personalizabil

### Sănătate și Întreținere

- **Motor de sănătate**: Verifică integritatea sistemului de fișiere, bazei de date, cache-ului și metadatelor
- **Reparare automată**: Reparare cu un click pentru problemele detectate
- **Recuperare sesiune**: Restaurează workspace-ul după închideri neașteptate
- **Mod somn**: Utilizare minimă a resurselor în stare inactivă

---

## Capturi de Ecran

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Interfață Principală" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfață Principală - Vizualizare Galerie</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Panou Inspector" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panou Inspector - Detalii Activ</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Interfață Căutare" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfață Căutare Avansată</em>
</p>

---

## Instalare

### Precondiții

- [Rust](https://www.rust-lang.org/tools/install) (cea mai recentă versiune stabilă)
- [Node.js](https://nodejs.org/) (v18 sau mai mare)
- [pnpm](https://pnpm.io/) (v8 sau mai mare)

### Descărcare

Descărcați cea mai recentă versiune de pe pagina [Releases](https://github.com/mh3nj/evoury/releases).

### Compilare din Sursă

```bash
# Clonați repository-ul
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instalați dependențele
pnpm install

# Porniți serverul de dezvoltare
pnpm tauri dev

# Compilați pentru producție
pnpm tauri build
```

---

## Dezvoltare

### Comenzi Disponibile

```bash
# Dezvoltare
pnpm dev              # Porniți serverul Vite
pnpm tauri dev        # Porniți Tauri în modul dezvoltare

# Compilare
pnpm build            # Compilați frontend-ul
pnpm tauri build      # Compilați aplicația Tauri pentru producție

# Teste
pnpm test             # Rulați testele frontend
cargo test            # Rulați testele Rust

# Lint
pnpm lint             # Rulați ESLint
cargo clippy          # Rulați Clippy

# Formatare
pnpm format           # Formatați codul frontend
cargo fmt             # Formatați codul Rust
```

---

## Stack Tehnologic

### Backend

- **Rust** - Limbaj de programare de sistem
- **Tauri** - Framework aplicații desktop
- **SQLite** - Bază de date locală
- **Crossbeam** - Primitive de concurență

### Frontend

- **React** - Bibliotecă UI
- **TypeScript** - JavaScript tip sigur
- **Tailwind CSS** - Framework CSS utilitar
- **Zustand** - Gestionare stare
- **Vite** - Instrument build și server dezvoltare

---

## Roadmap

Vedeți [ROADMAP.md](ROADMAP.md) pentru roadmap-ul detaliat de dezvoltare.

---

## Contribuire

Contribuțiile sunt binevenite! Vă rugăm să citiți mai întâi [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licență

Acest proiect este licențiat sub licența MIT - vezi fișierul [LICENSE](LICENSE) pentru detalii.

---

## Suport

- **Probleme**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discuții**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Făcut cu ❤️ de <a href="https://github.com/mh3nj">Numele Tău</a>
</p>
