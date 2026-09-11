<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Gestore di Asset Creativi Offline</strong>
</p>

<p align="center">
  <a href="#features">Funzionalità</a> •
  <a href="#installation">Installazione</a> •
  <a href="#development">Sviluppo</a> •
  <a href="#architecture">Architettura</a> •
  <a href="#contributing">Contribuire</a> •
  <a href="#license">Licenza</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versione">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licenza">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Piattaforma">
</p>

---

## Informazioni

Evoury è un potente gestore di asset creativi offline, costruito con Tauri, React e Rust. Progettato per i professionisti creativi che necessitano di accesso rapido e affidabile ai propri asset digitali senza compromessi su prestazioni e privacy.

### Perché Evoury?

- **Offline**: I tuoi asset rimangono sulla tua macchina. Nessuna dipendenza dal cloud.
- **Ultra veloce**: Costruito con Rust per prestazioni che scalano con la tua libreria.
- **Architettura modulare**: Più di 40 crate specializzati per massima flessibilità.
- **Interfaccia bella**: Interfaccia moderna e responsive costruita con React e Tailwind CSS.

---

## Funzionalità

### Motore Principale

- **Supporto multi-formato**: Immagini, video, modelli 3D, audio, documenti e altro
- **Appaiamento intelligente**: Raggruppa automaticamente i file correlati
- **Macchina degli stati**: Traccia gli asset dalla scoperta all'archiviazione
- **Architettura ad eventi**: Comunicazione decollata tramite bus degli eventi

### Gestione delle Librerie

- **Scanner avanzato**: Modalità di scansione completa, incrementale, per cartella e in background
- **Osservatore del filesystem**: Sincronizzazione in tempo reale senza aggiornamento manuale
- **Pipeline dei metadati**: Estrazione, normalizzazione, validazione e cache automatiche
- **Rilevamento duplicati**: SHA256, hash percettivo e basato sui metadati

### Ricerca e Organizzazione

- **Indice di ricerca persistente**: Ricerca full-text ultra-veloce con FTS5
- **Collezioni intelligenti**: Collezioni basate su regole con aggiornamento automatico
- **Linguaggio di query avanzato**: Filtri per tipo, tag, valutazione, data, fotocamera e altro
- **Profili di ricerca**: Salva e passa tra le configurazioni di ricerca

### Sistema Workspace

- **Workspace persistenti**: Ricorda l'intero stato della sessione
- **Workspace multipli**: Passa tra diversi contesti del progetto
- **Workstation**: Layout preconfigurati con strumenti, scorciatoie e temi
- **Pannelli dockabili**: Motore di layout completamente personalizzabile

### Salute e Manutenzione

- **Motore di salute**: Verifica l'integrità del filesystem, database, cache e metadati
- **Riparazione automatica**: Riparazione con un clic per i problemi rilevati
- **Recupero sessione**: Ripristina il workspace dopo spegnimenti imprevisti
- **Modalità sonno**: Uso minimo delle risorse inattivo

---

## Screenshot

<p align="center">
  <img src="public/images/main_dark.webp" alt="Interfaccia Principale" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfaccia Principale - Vista Galleria</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Pannello Ispettore" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Pannello Ispettore - Dettagli Asset</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Interfaccia Ricerca" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfaccia Ricerca Avanzata</em>
</p>

---

## Installazione

### Prerequisiti

- [Rust](https://www.rust-lang.org/tools/install) (ultima versione stabile)
- [Node.js](https://nodejs.org/) (v18 o superiore)
- [pnpm](https://pnpm.io/) (v8 o superiore)

### Download

Scarica l'ultima versione dalla pagina [Releases](https://github.com/mh3nj/evoury/releases).

### Compila dal Codice Sorgente

```bash
# Clona il repository
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installa le dipendenze
pnpm install

# Avvia il server di sviluppo
pnpm tauri dev

# Compila per la produzione
pnpm tauri build
```

---

## Sviluppo

### Comandi Disponibili

```bash
# Sviluppo
pnpm dev              # Avvia il server Vite
pnpm tauri dev        # Avvia Tauri in modalità sviluppo

# Compilazione
pnpm build            # Compila il frontend
pnpm tauri build      # Compila l'app Tauri per la produzione

# Test
pnpm test             # Esegui i test frontend
cargo test            # Esegui i test Rust

# Lint
pnpm lint             # Esegui ESLint
cargo clippy          # Esegui Clippy

# Formattazione
pnpm format           # Formatta il codice frontend
cargo fmt             # Formatta il codice Rust
```

---

## Stack Tecnologico

### Backend

- **Rust** - Linguaggio di programmazione di sistema
- **Tauri** - Framework per applicazioni desktop
- **SQLite** - Database locale
- **Crossbeam** - Primitive di programmazione concorrente

### Frontend

- **React** - Libreria UI
- **TypeScript** - JavaScript tipizzato
- **Tailwind CSS** - Framework CSS utility-first
- **Zustand** - Gestione dello stato
- **Vite** - Strumento di build e server di sviluppo

---

## Roadmap

Vedi [ROADMAP.md](ROADMAP.md) per la roadmap dettagliata dello sviluppo.

---

## Contribuire

I contributi sono i benvenuti! Si prega di leggere prima [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenza

Questo progetto è con licenza MIT - vedi il file [LICENSE](LICENSE) per i dettagli.

---

## Supporto

- **Problemi**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discussioni**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Fatto con ❤️ da <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
