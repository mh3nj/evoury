<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-First Kreativní Správce Aktiv</strong>
</p>

<p align="center">
  <a href="#features">Funkce</a> •
  <a href="#installation">Instalace</a> •
  <a href="#development">Vývoj</a> •
  <a href="#architecture">Architektura</a> •
  <a href="#contributing">Přispívání</a> •
  <a href="#license">Licence</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Verze">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licence">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforma">
</p>

---

## O Projektu

Evoury je výkonný, offline-first správce kreativních aktiv postavený pomocí Tauri, React a Rust. Navržený pro kreativní profesionály, kteří potřebují rychlý a spolehlivý přístup ke svým digitálním aktivům bez kompromisů ve výkonu nebo soukromí.

### Proč Evoury?

- **Offline-first**: Vaše aktiva zůstanou na vašem stroji. Žádná závislost na cloudu.
- **Bleskově rychlý**: Postavený pomocí Rust pro výkon, který škáluje s vaší knihovnou.
- **Modulární architektura**: Více než 40 specializovaných crateů pro maximální flexibilitu.
- **Krásné UI**: Moderní, responzivní rozhraní postavené pomocí React a Tailwind CSS.

---

## Funkce

### Hlavní motor

- **Multi-formátová podpora**: Obrázky, videa, 3D modely, audio, dokumenty a více
- **Chytré párování**: Automaticky grupuje související soubory
- **Stroj stavů aktiv**: Sleduje aktiva od objevení po archivaci
- **Událostmi řízená architektura**: Oddělená komunikace přes sběrnici událostí

### Správa knihoven

- **Pokročilý skener**: Plné, přírůstkové, složkově-specifické a pozadí skenovací režimy
- **Pozorovatel souborového systému**: Real-time synchronizace bez ručního obnovení
- **Metadata pipeline**: Automatická extrakce, normalizace, validace a cacheování
- **Detekce duplicit**: SHA256, percepční hašování a metadata-založená

### Vyhledávání a organizace

- **Persistantní vyhledávací index**: Bleskově rychlé plnotextové vyhledávání s FTS5
- **Chytré kolekce**: Pravidly založené automaticky aktualizované kolekce
- **Pokročilý jazyk dotazů**: Filtry podle typu, tagu, hodnocení, data, fotoaparátu a více
- **Vyhledávací profily**: Uložte a přepínejte mezi konfiguracemi vyhledávání

### Systém pracovního prostoru

- **Persistantní pracovní prostory**: Pamatuje celý stav relace
- **Více pracovních prostorů**: Přepínání mezi různými kontexty projektů
- **Pracovní stanice**: Předkonfigurovaná rozložení s nástroji, zkratkami a motivy
- **Dokovatelné panely**: Plně přizpůsobitelný motor rozložení

### Zdraví a údržba

- **Zdravotní motor**: Kontroluje integritu souborového systému, databáze, cache a metadat
- **Automatická oprava**: Oprava jedním kliknutím pro detekované problémy
- **Obnovení relace**: Obnoví pracovní prostor po neočekávaném vypnutí
- **Režim spánku**: Minimální využití zdrojů v nečinnosti

---

## Snímky obrazovky

<p align="center">
  <img src="public/images/main_dark.webp" alt="Hlavní rozhraní" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Hlavní rozhraní - Zobrazení galerie</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Panel inspektoru" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel inspektoru - Podrobnosti aktiva</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Vyhledávací rozhraní" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Pokročilé vyhledávací rozhraní</em>
</p>

---

## Instalace

### Předpoklady

- [Rust](https://www.rust-lang.org/tools/install) (nejnovější stabilní verze)
- [Node.js](https://nodejs.org/) (v18 nebo novější)
- [pnpm](https://pnpm.io/) (v8 nebo novější)

### Stažení

Stáhněte nejnovější verzi ze stránky [Releases](https://github.com/mh3nj/evoury/releases).

### Sestavení ze zdrojového kódu

```bash
# Klonujte repozitář
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Nainstalujte závislosti
pnpm install

# Spusťte vývojový server
pnpm tauri dev

# Sestavte pro produkci
pnpm tauri build
```

---

## Vývoj

### Dostupné příkazy

```bash
# Vývoj
pnpm dev              # Spusťte Vite vývojový server
pnpm tauri dev        # Spusťte Tauri ve vývojovém režimu

# Sestavení
pnpm build            # Sestavte frontend
pnpm tauri build      # Sestavte Tauri aplikaci pro produkci

# Testy
pnpm test            # Spusťte frontend testy
cargo test           # Spusťte Rust testy

# Lint
pnpm lint            # Spusťte ESLint
cargo clippy         # Spusťte Clippy

# Formátování
pnpm format          # Formátujte frontend kód
cargo fmt            # Formátujte Rust kód
```

---

## Tech Stack

### Backend

- **Rust** - Systémový programovací jazyk
- **Tauri** - Framework desktopových aplikací
- **SQLite** - Lokální databáze
- **Crossbeam** - Primitivy pro souběžnost

### Frontend

- **React** - UI knihovna
- **TypeScript** - Typově bezpečný JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Správa stavu
- **Vite** - Build nástroj a vývojový server

---

## Mapa cesty

Podrobnou mapu vývoje naleznete v [ROADMAP.md](ROADMAP.md).

---

## Přispívání

Příspěvky jsou vítány! Přečtěte si prosím nejprve [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licence

Tento projekt je licencován pod MIT licencí - podrobnosti viz soubor [LICENSE](LICENSE).

---

## Podpora

- **Problémy**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskuze**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Vytvořeno s ❤️ od <a href="https://github.com/mh3nj">Vaše jméno</a>
</p>
