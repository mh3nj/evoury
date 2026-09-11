<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-first Kreativ-Asset-Manager</strong>
</p>

<p align="center">
  <a href="#features">Funktionen</a> •
  <a href="#installation">Installation</a> •
  <a href="#development">Entwicklung</a> •
  <a href="#architecture">Architektur</a> •
  <a href="#contributing">Beitragen</a> •
  <a href="#license">Lizenz</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lizenz">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plattform">
</p>

---

## Über

Evoury ist ein leistungsstarker, offline-first Kreativ-Asset-Manager, der mit Tauri, React und Rust gebaut wurde. Entwickelt für kreative Profis, die schnellen und zuverlässigen Zugriff auf ihre digitalen Assets benötigen, ohne Leistung oder Privatsphäre zu kompromittieren.

### Warum Evoury?

- **Offline-first**: Ihre Assets bleiben auf Ihrem Computer. Keine Cloud-Abhängigkeit.
- **Blitzschnell**: Mit Rust gebaut für Leistung, die mit Ihrer Bibliothek skaliert.
- **Modulare Architektur**: 40+ spezialisierte Crates für maximale Flexibilität.
- **Schönes UI**: Modernes, responsive Interface mit React und Tailwind CSS.

---

## Funktionen

### Hauptmotor

- **Mehrformat-Unterstützung**: Bilder, Videos, 3D-Modelle, Audio, Dokumente und mehr
- **Smartes Asset-Paarung**: Gruppiert automatisch zusammengehörige Dateien
- **Asset-Zustandsmaschine**: Verfolgt Assets von der Entdeckung bis zur Archivierung
- **Ereignisgesteuerte Architektur**: Entkoppelte Kommunikation über Event-Bus

### Bibliotheksverwaltung

- **Erweiterter Scanner**: Voll-, inkrementeller, Ordner- und Hintergrund-Scan-Modi
- **Dateisystem-Beobachter**: Echtzeit-Synchronisation ohne manuelles Aktualisieren
- **Metadaten-Pipeline**: Automatische Extraktion, Normalisierung, Validierung und Caching
- **Duplikat-Erkennung**: SHA256, perceptual hashing und metadatenbasierte Erkennung

### Suche und Organisation

- **Persistentes Suchindex**: Blitzschnelle Volltextsuche mit FTS5
- **Smart Collections**: Regelnbasierte automatisch aktualisierende Sammlungen
- **Erweiterte Abfragesprache**: Filter nach Typ, Tag, Bewertung, Datum, Kamera und mehr
- **Suchprofile**: Speichern und Wechseln zwischen Suchkonfigurationen

### Workspace-System

- **Persistente Workspaces**: Erinnert sich an den gesamten Sitzungszustand
- **Mehrere Workspaces**: Wechsel zwischen verschiedenen Projektkontexten
- **Workstations**: Vorkonfigurierte Layouts mit Werkzeugen, Shortcuts und Themes
- **Dokbare Panels**: Vollständig anpassbares Layout-System

### Gesundheit und Wartung

- **Gesundheitsmotor**: Prüft Dateisystem, Datenbank, Cache und Metadaten-Integrität
- **Automatische Reparatur**: Ein-Klick-Reparatur für erkannte Probleme
- **Sitzungswiederherstellung**: Stellt Workspace nach unerwarteten Abstürzen wieder her
- **Schlafmodus**: Minimale Ressourcennutzung im Leerlauf

---

## Screenshots

<p align="center">
  <img src="public/images/main_dark.webp" alt="Hauptinterface" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hauptinterface - Galerieansicht</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspektor-Panel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspektor-Panel - Asset-Details</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Suchoberfläche" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Erweiterte Suchoberfläche</em>
</p>

---

## Installation

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (neueste stabile Version)
- [Node.js](https://nodejs.org/) (v18 oder höher)
- [pnpm](https://pnpm.io/) (v8 oder höher)

### Download

Laden Sie die neueste Version von der [Releases](https://github.com/mh3nj/evoury/releases)-Seite herunter.

### Aus Quellcode bauen

```bash
# Repository klonen
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Abhängigkeiten installieren
pnpm install

# Entwicklungsserver starten
pnpm tauri dev

# Production-Build
pnpm tauri build
```

---

## Entwicklung

### Verfügbare Befehle

```bash
# Entwicklung
pnpm dev              # Vite-Entwicklungsserver starten
pnpm tauri dev        # Tauri im Entwicklungsmodus starten

# Build
pnpm build            # Frontend bauen
pnpm tauri build      # Tauri-App für Produktion bauen

# Tests
pnpm test             # Frontend-Tests ausführen
cargo test            # Rust-Tests ausführen

# Lint
pnpm lint             # ESLint ausführen
cargo clippy          # Clippy ausführen

# Formatierung
pnpm format           # Frontend-Code formatieren
cargo fmt             # Rust-Code formatieren
```

---

## Technologie-Stack

### Backend

- **Rust** - Systemprogrammiersprache
- **Tauri** - Desktop-Anwendungs-Framework
- **SQLite** - Lokale Datenbank
- **Crossbeam** - Nebenläufigkeitsprimitiven

### Frontend

- **React** - UI-Bibliothek
- **TypeScript** - Typsicheres JavaScript
- **Tailwind CSS** - Utility-first CSS-Framework
- **Zustand** - Zustandsverwaltung
- **Vite** - Build-Tool und Entwicklungsserver

---

## Roadmap

Siehe [ROADMAP.md](ROADMAP.md) für die detaillierte Entwicklungsroadmap.

---

## Beitrragen

Beiträge sind willkommen! Bitte lesen Sie zuerst [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz - siehe [LICENSE](LICENSE) für Details.

---

## Support

- **Probleme**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskussionen**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Mit ❤️ gemacht von <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
