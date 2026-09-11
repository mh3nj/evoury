<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline i Parë Menaxher Kreativ i Pasurive</strong>
</p>

<p align="center">
  <a href="#features">Funksionet</a> •
  <a href="#installation">Instalimi</a> •
  <a href="#development">Zhvillimi</a> •
  <a href="#architecture">Arkitektura</a> •
  <a href="#contributing">Kontributi</a> •
  <a href="#license">Licenca</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versioni">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licenca">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforma">
</p>

---

## Për Projektin

Evoury është një menaxher i fuqishëm, offline i parë i pasurive kreative i ndërtuar me Tauri, React dhe Rust. Dizajnuar për profesionistët krijues që kanë nevojë për qasje të shpejtë dhe të besueshme në pasuritë e tyre dixhitale pa kompromente në performancën ose privatësinë.

### Pse Evoury?

- **Offline i parë**: Pasuritë tuaja mbeten në kompjuterin tuaj. Nuk ka varësi nga reja.
- **I shpejtë si rrufeja**: Ndërtuar me Rust për performancë që shkallet me bibliotekën tuaj.
- **Arkitekturë modulare**: Më shumë se 40 crate të specializuara për fleksibilitet maksimal.
- **UI e bukur**: Ndërfaqe moderne, responsive e ndërtuar me React dhe Tailwind CSS.

---

## Funksionet

### Motori Kryesor

- **Mbështetje multi-format**: Imazhe, video, modele 3D, audio, dokumente dhe më shumë
- **Përzierje e zgjuar**: Grupon automatikisht skedarët e lidhur
- **Makineri e gjendjeve**: Gjurmon pasuritë nga zbulimi deri te arkivimi
- **Arkitekturë e udhëhequr nga ngjarjet**: Komunikim i ndarë përmes bus-it të ngjarjeve

### Menaxhimi i Bibliotekave

- **Skener i avancuar**: Modet e skanimit të plotë, inkremental, të përcaktuar për dosje dhe në sfond
- **Vëzhgues i sistemit të skedarëve**: Sinkronizim në kohë reale pa rifreskim manual
- **Tub i metadata**: Eksktraction, normalizim, vlefshmëri dhe cache automatike
- **Zbulim i dublikatëve**: SHA256, hash perceptual dhe i bazuar në metadata

### Kërkimi dhe Organizimi

- **Indeks kërkimi i përhershëm**: Kërkim i plotë me shpejtësi rrufeje me FTS5
- **Koleksione të zgjuara**: Koleksione të bazuar në rregulla me përditësim automatik
- **Gjuhë e avancuar e kërkesave**: Filtra sipas tipit, etiketës, vlerësimit, datës, kamerës dhe më shumë
- **Profile kërkimi**: Ruani dhe kaloni midis konfigurimeve të kërkimit

### Sistemi i Hapësirës së Punës

- **Hapësira pune të përhershme**: Kujton gjendjen e plotë të seancës
- **Hapësira të shumta pune**: Kaloni midis konteksteve të ndryshme të projekteve
- **Stacione pune**: Layout të parakonfiguruar me mjete, shkurtore dhe tema
- **Panel të ankorueshëm**: Motor layout plotësisht i personalizueshëm

### Shëndeti dhe Mirëmbajtja

- **Motor shëndeti**: Kontrollon integritetin e sistemit të skedarëve, bazës së të dhënave, cache-it dhe metadata
- **Riparim automatik**: Riparim me një klik për problemet e zbuluara
- **Rikuperim seance**: Rivendos hapësirën e punës pas mbylljeve të papritura
- **Modalitet gjumi**: Përdorim minimal i burimeve në pushim

---

## Screenshots

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Ndërfaqja Kryesore" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Ndërfaqja Kryesore - Pamja e Galerisë</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Paneli i Inspektorit" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Paneli i Inspektorit - Detajet e Pasurisë</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Ndërfaqja e Kërkimit" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Ndërfaqja e Kërkimit të Avancuar</em>
</p>

---

## Instalimi

### Parakushtet

- [Rust](https://www.rust-lang.org/tools/install) (versioni më i fundit i qëndrueshëm)
- [Node.js](https://nodejs.org/) (v18 ose më i ri)
- [pnpm](https://pnpm.io/) (v8 ose më i ri)

### Shkarkimi

Shkarkoni versionin e fundit nga faqja [Releases](https://github.com/mh3nj/evoury/releases).

### Ndërtimi nga Burimi

```bash
# Klononi repozitorin
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instaloni varësitë
pnpm install

# Filloni serverin e zhvillimit
pnpm tauri dev

# Ndërtoni për prodhim
pnpm tauri build
```

---

## Zhvillimi

### Komandat e Disponueshme

```bash
# Zhvillimi
pnpm dev              # Filloni serverin Vite
pnpm tauri dev        # Filloni Tauri në modalitetin e zhvillimit

# Ndërtimi
pnpm build            # Ndërtoni frontend
pnpm tauri build      # Ndërtoni aplikacionin Tauri për prodhim

# Testet
pnpm test             # Ekzekutoni testet e frontend
cargo test            # Ekzekutoni testet e Rust

# Lint
pnpm lint             # Ekzekutoni ESLint
cargo clippy          # Ekzekutoni Clippy

# Formatosimi
pnpm format           # Formatosni kodin e frontend
cargo fmt             # Formatosni kodin e Rust
```

---

## Stack Teknologjik

### Backend

- **Rust** - Gjuhë e programimit të sistemit
- **Tauri** - Framework i aplikacionit desktop
- **SQLite** - Baza e të dhënave lokale
- **Crossbeam** - Primitivë të konkurrencës

### Frontend

- **React** - Librari UI
- **TypeScript** - JavaScript i sigurt me tipa
- **Tailwind CSS** - Framework CSS utility-first
- **Zustand** - Menaxhimi i gjendjes
- **Vite** - Mjet ndërtimi dhe server zhvillimi

---

## Harta e Rrugës

Shikoni [ROADMAP.md](ROADMAP.md) për harten e detajuar të zhvillimit.

---

## Kontributi

Kontributet janë të mirëseardhura! Ju lutemi lexoni fillimisht [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenca

Ky projekt është licencuar nën licencën MIT - shikoni skedarin [LICENSE](LICENSE) për detaje.

---

## Mbështetja

- **Probleme**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskutime**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Bërë me ❤️ nga <a href="https://github.com/mh3nj">Emri Juaj</a>
</p>
