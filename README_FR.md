<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Gestionnaire d'Actifs Créatifs Hors Ligne</strong>
</p>

<p align="center">
  <a href="#features">Fonctionnalités</a> •
  <a href="#installation">Installation</a> •
  <a href="#development">Développement</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#contributing">Contribuer</a> •
  <a href="#license">Licence</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licence">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plateforme">
</p>

---

## À Propos

Evoury est un gestionnaire d'actifs créatifs puissant et hors ligne, construit avec Tauri, React et Rust. Conçu pour les professionnels créatifs qui ont besoin d'un accès rapide et fiable à leurs actifs numériques sans compromettre les performances ou la confidentialité.

### Pourquoi Evoury ?

- **Hors ligne** : Vos actifs restent sur votre machine. Pas de dépendance au cloud.
- **Ultra rapide** : Construit avec Rust pour des performances qui s'adaptent à votre bibliothèque.
- **Architecture modulaire** : Plus de 40 crates spécialisés pour une flexibilité maximale.
- **Interface magnifique** : Interface moderne et réactive construite avec React et Tailwind CSS.

---

## Fonctionnalités

### Moteur Principal

- **Support multi-formats** : Images, vidéos, modèles 3D, audio, documents et plus
- **Appariement intelligent** : Grouppe automatiquement les fichiers liés
- **Machine d'états** : Suit les actifs de la découverte à l'archivage
- **Architecture événementielle** : Communication découplée via bus d'événements

### Gestion des Bibliothèques

- **Scanner avancé** : Modes de numérisation complète, incrémentale, par dossier et en arrière-plan
- **Observateur du système de fichiers** : Synchronisation en temps réel sans actualisation manuelle
- **Pipeline de métadonnées** : Extraction, normalisation, validation et mise en cache automatiques
- **Détection de doublons** : SHA256, hachage perceptuel et basé sur les métadonnées

### Recherche et Organisation

- **Index de recherche persistant** : Recherche plein texte ultra-rapide avec FTS5
- **Collections intelligentes** : Collections basées sur des règles avec mise à jour automatique
- **Langage de requête avancé** : Filtres par type, étiquette, note, date, caméra et plus
- **Profils de recherche** : Enregistrez et basculez entre les configurations de recherche

### Système d'Espaces de Travail

- **Espaces de travail persistants** : Se souvient de tout l'état de la session
- **Plusieurs espaces de travail** : Basculez entre différents contextes de projet
- **Stations de travail** : Mises en page préconfigurées avec outils, raccourcis et thèmes
- **Panels ancrables** : Moteur de mise en page entièrement personnalisable

### Santé et Maintenance

- **Moteur de santé** : Vérifie l'intégrité du système de fichiers, de la base de données, du cache et des métadonnées
- **Réparation automatique** : Réparation en un clic pour les problèmes détectés
- **Récupération de session** : Restaure l'espace de travail après des arrêts inattendus
- **Mode veille** : Utilisation minimale des ressources au repos

---

## Captures d'Écran

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Interface Principale" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interface Principale - Vue Galerie</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Panneau d'Inspection" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panneau d'Inspection - Détails de l'Actif</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Interface de Recherche" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interface de Recherche Avancée</em>
</p>

---

## Installation

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (dernière version stable)
- [Node.js](https://nodejs.org/) (v18 ou supérieur)
- [pnpm](https://pnpm.io/) (v8 ou supérieur)

### Téléchargement

Téléchargez la dernière version depuis la page [Releases](https://github.com/mh3nj/evoury/releases).

### Compiler depuis les Sources

```bash
# Cloner le dépôt
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Installer les dépendances
pnpm install

# Démarrer le serveur de développement
pnpm tauri dev

# Compiler pour la production
pnpm tauri build
```

---

## Développement

### Commandes Disponibles

```bash
# Développement
pnpm dev              # Démarrer le serveur Vite
pnpm tauri dev        # Démarrer Tauri en mode développement

# Compilation
pnpm build            # Compiler le frontend
pnpm tauri build      # Compiler l'application Tauri

# Tests
pnpm test             # Exécuter les tests frontend
cargo test            # Exécuter les tests Rust

# Lint
pnpm lint             # Exécuter ESLint
cargo clippy          # Exécuter Clippy

# Formatage
pnpm format           # Formater le code frontend
cargo fmt             # Formater le code Rust
```

---

## Stack Technique

### Backend

- **Rust** - Langage de programmation système
- **Tauri** - Framework d'application de bureau
- **SQLite** - Base de données locale
- **Crossbeam** - Primitives de programmation concurrente

### Frontend

- **React** - Bibliothèque UI
- **TypeScript** - JavaScript typé
- **Tailwind CSS** - Framework CSS utilitaire
- **Zustand** - Gestion d'état
- **Vite** - Outil de compilation et serveur de développement

---

## Feuille de Route

Voir [ROADMAP.md](ROADMAP.md) pour la feuille de route détaillée du développement.

---

## Contribuer

Les contributions sont les bienvenues ! Veuillez d'abord lire [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

## Support

- **Problèmes** : [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discussions** : [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Fait avec ❤️ par <a href="https://github.com/mh3nj">Votre Nom</a>
</p>
