<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Frakoblet Først Kreativ Tillgangshåndtering</strong>
</p>

<p align="center">
  <a href="#features">Funksjoner</a> •
  <a href="#installation">Installasjon</a> •
  <a href="#development">Utvikling</a> •
  <a href="#architecture">Arkitektur</a> •
  <a href="#contributing">Bidra</a> •
  <a href="#license">Lisens</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versjon">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lisens">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plattform">
</p>

---

## Om

Evoury er en kraftfull, frakoblet først kreativ tillgangshåndtering bygget med Tauri, React og Rust. Designet for kreative profesjonelle som trenger rask og pålitelig tilgang til sine digitale eiendeler uten å kompromittere ytelse eller personvern.

### Hvorfor Evoury?

- **Frakoblet først**: Dine eiendeler forblir på din maskin. Ingen skyavhengighet.
- **Lynrask**: Bygget med Rust for ytelse som skalerer med ditt bibliotek.
- **Modulær arkitektur**: 40+ spesialiserte cates for maksimal fleksibilitet.
- **Vakkert grensesnitt**: Moderat, responsivt grensesnitt bygget med React og Tailwind CSS.

---

## Funksjoner

### Kjerne motor

- **Multi-format støtte**: Bilder, videoer, 3D-modeller, lyd, dokumenter og mer
- **Smart sammenkobling**: Grupperer automatisk relaterte filer
- **Eiendelstilstandsmaskin**: Sporer eiendeler fra oppdagelse til arkivering
- **Hendelsedrevet arkitektur**: Frakoblet kommunikasjon gjennom hendelsesbuss

### Bibliotekhåndtering

- **Avansert skanner**: Fullstendig, inkrementell, mappe-spesifikk og bakgrunnsskanningsmoduser
- **Filsystemovervåker**: Sanntidssynkronisering uten manuell oppdatering
- **Metadata-rørledning**: Automatisk ekstrahering, normalisering, validering og buffering
- **Duplikatdeteksjon**: SHA256, oppfatningsbasert hashing og metadata-basert

### Søk og organisering

- **Permanent søkeindeks**: Lynrask fulltekstsøk med FTS5
- **Smarte samlinger**: Regelbaserte automatisk oppdaterte samlinger
- **Avansert spørrespråk**: Filtrering etter type, tagg, vurdering, dato, kamera og mer
- **Søkeprofiler**: Lagre og veksle mellom søkekonfigurasjoner

### Arbeidsplass-system

- **Permanente arbeidsplasser**: Husker hele økttilstanden
- **Flere arbeidsplasser**: Veksle mellom ulike prosjektkontekster
- **Arbeidsstasjoner**: Forhåndskonfigurerte oppsett med verktøy, snarveier og temaer
- **Dokkbare paneler**: Fullt tilpasselig oppsettmotor

### Helse og vedlikehold

- **Helsemotor**: Sjekker integriteten til filsystem, database, buffer og metadata
- **Automatisk reparasjon**: Enklikks reparasjon for oppdagede problemer
- **Øktgjenoppretting**: Gjenoppretter arbeidsplassen etter uventede avslutninger
- **Hvilemodus**: Minimal ressursbruk ved inaktivitet

---

## Skjermbilder

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Hovedgrensesnitt" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hovedgrensesnitt - Gallerivisning</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Inspektørpanel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspektørpanel - Eiendelsdetaljer</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Søkegrensesnitt" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Avansert søkegrensesnitt</em>
</p>

---

## Installasjon

### Forutsetninger

- [Rust](https://www.rust-lang.org/tools/install) (siste stabile versjon)
- [Node.js](https://nodejs.org/) (v18 eller nyere)
- [pnpm](https://pnpm.io/) (v8 eller nyere)

### Nedlasting

Last ned den siste versjonen fra [Releases](https://github.com/mh3nj/evoury/releases) siden.

### Bygg fra kildekode

```bash
# Klon repot
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installer avhengigheter
pnpm install

# Start utviklingsserveren
pnpm tauri dev

# Bygg for produksjon
pnpm tauri build
```

---

## Utvikling

### Tilgjengelige kommandoer

```bash
# Utvikling
pnpm dev              # Start Vite-utviklingsserveren
pnpm tauri dev        # Start Tauri i utviklingsmodus

# Bygg
pnpm build            # Bygg frontend
pnpm tauri build      # Bygg Tauri-appen for produksjon

# Tester
pnpm test             # Kjør frontend-tester
cargo test            # Kjør Rust-tester

# Lint
pnpm lint             # Kjør ESLint
cargo clippy          # Kjør Clippy

# Formatering
pnpm format           # Formater frontend-kode
cargo fmt             # Formater Rust-kode
```

---

## Teknisk stable

### Backend

- **Rust** - Systemprogrammeringsspråk
- **Tauri** - Skrivebordsapplikasjonsrammeverk
- **SQLite** - Lokal database
- **Crossbeam** - Konkurransprimitiver

### Frontend

- **React** - UI-bibliotek
- **TypeScript** - Typsikkert JavaScript
- **Tailwind CSS** - Verktøybasert CSS-rammeverk
- **Zustand** - Tilstandshåndtering
- **Vite** - Byggverktøy og utviklingsserver

---

## Veikart

Se [ROADMAP.md](ROADMAP.md) for detaljert utviklingsveikart.

---

## Bidra

Bidrag er velkomne! Vennligst les [CONTRIBUTING.md](CONTRIBUTING.md) først.

---

## Lisens

Dette prosjektet er lisensiert under MIT-lisensen - se filen [LICENSE](LICENSE) for detaljer.

---

## Støtte

- **Problemer**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskusjoner**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Laget med ❤️ av <a href="https://github.com/mh3nj">Ditt navn</a>
</p>
