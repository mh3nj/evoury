<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-Először Kreatív Eszközkezelő</strong>
</p>

<p align="center">
  <a href="#features">Jellemzők</a> •
  <a href="#installation">Telepítés</a> •
  <a href="#development">Fejlesztés</a> •
  <a href="#architecture">Architektúra</a> •
  <a href="#contributing">Közreműködés</a> •
  <a href="#license">Licenc</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Verzió">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licenc">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Róla

Evoury egy erős, offline-először kreatív eszközkezelő, amely Tauri, React és Rust segítségével készült. Kreatív szakemberek számára tervezték, akiknek gyors és megbízható hozzáférésre van szükségük digitális eszközeikhez anélkül, hogy kompromisszumot kötnének a teljesítményben vagy a magánéletben.

### Miért Evoury?

- **Offline-először**: Eszközeid a gépeden maradnak. Nincs felhőfüggőség.
- **Villámgyors**: Rust-tal építve a teljesítményért, amely méretezhető a könyvtárad méretével.
- **Moduláris architektúra**: 40+ specializált crate a maximális rugalmasságért.
- **Gyönyörű UI**: Modern, reszponzív felület, React-tal és Tailwind CSS-sel építve.

---

## Jellemzők

### Mag Motor

- **Többformátumú támogatás**: Képek, videók, 3D modellek, hang, dokumentumok és több
- **Okos párosítás**: Automatikusan csoportosítja a kapcsolódó fájlokat
- **Eszköz állapotgép**: Nyomon követi az eszközöket a felfedezéstől az archiválásig
- **Eseményvezérelt architektúra**: Lazán csatolt kommunikáció eseménybuszon keresztül

### Könyvtárkezelés

- **Fejlett szkenner**: Teljes, inkrementális, mappaspecifikus és háttér szkennelési módok
- **Fájlrendszer figyelő**: Valós idejű szinkronizáció kézi frissítés nélkül
- **Metaadat pipeline**: Automatikus kinyerés, normalizálás, érvényesítés és gyorsítótárazás
- **Duplikátum észlelés**: SHA256, észleleti hash és metaadatalapú

### Keresés és Szervezés

- **Állandó keresési index**: Villámgyors teljes szöveges keresés FTS5-tel
- **Okos gyűjtemények**: Szabályalapú, automatikusan frissülő gyűjtemények
- **Fejlett lekérdezési nyelv**: Szűrők típus, címke, értékelés, dátum, kamera és több szerint
- **Keresési profilok**: Mentés és váltás keresési konfigurációk között

### Munkaterület Rendszer

- **Állandó munkaterületek**: Megjegyzi az egész munkamenet állapotát
- **Több munkaterület**: Váltás különböző projekt kontextusok között
- **Munkaállomások**: Előre konfigurált elrendezések eszközökkel, billentyűparancsokkal és témákkal
- **Dokkolható panelek**: Teljesen testreszabható elrendezés motor

### Egészség és Karbantartás

- **Egészség motor**: Ellenőrzi a fájlrendszer, adatbázis, gyorsítótár és metaadatok integritását
- **Automatikus javítás**: Kattintásos javítás a észlelt problémákra
- **Munkamenet visszaállítás**: Visszaállítja a munkaterületet váratlan leállítások után
- **Alvó mód**: Minimális erőforrás felhasználás tétlen állapotban

---

## Képernyőképek

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Fő felület" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Fő felület - Galéria nézet</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Vizsgáló panel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Vizsgáló panel - Eszköz részletek</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Keresési felület" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Fejlett keresési felület</em>
</p>

---

## Telepítés

### Előfeltételek

- [Rust](https://www.rust-lang.org/tools/install) (legfrissebb stabil verzió)
- [Node.js](https://nodejs.org/) (v18 vagy újabb)
- [pnpm](https://pnpm.io/) (v8 vagy újabb)

### Letöltés

Töltse le a legfrissebb verziót a [Releases](https://github.com/mh3nj/evoury/releases) oldalról.

### Forráskódból telepítés

```bash
# Klónozza a tárhelyet
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Telepíti a függőségeket
pnpm install

# Elindítja a fejlesztő szervert
pnpm tauri dev

# Építés termelésre
pnpm tauri build
```

---

## Fejlesztés

### Elérhető parancsok

```bash
# Fejlesztés
pnpm dev              # Elindítja a Vite fejlesztő szervert
pnpm tauri dev        # Elindítja a Tauri-t fejlesztő módban

# Építés
pnpm build            # Építi a frontendet
pnpm tauri build      # Építi a Tauri alkalmazást termelésre

# Tesztelés
pnpm test             # Futtatja a frontend teszteket
cargo test            # Futtatja a Rust teszteket

# Lint
pnpm lint             # Futtatja az ESLint-ot
cargo clippy          # Futtatja a Clippy-t

# Formázás
pnpm format           # Formázza a frontend kódot
cargo fmt             # Formázza a Rust kódot
```

---

## Technológiai Verem

### Backend

- **Rust** - Rendszer programozási nyelv
- **Tauri** - Asztali alkalmazás keretrendszer
- **SQLite** - Helyi adatbázis
- **Crossbeam** - Párhuzamossági primitívek

### Frontend

- **React** - UI könyvtár
- **TypeScript** - Típusbiztos JavaScript
- **Tailwind CSS** - Eszközelőny CSS keretrendszer
- **Zustand** - Állapotkezelés
- **Vite** - Építőeszköz és fejlesztő szerver

---

## Útiterv

Részletes fejlesztési útitervért lásd a [ROADMAP.md](ROADMAP.md) fájlt.

---

## Közreműködés

A közreműködés üdvözlendő! Kérjük, olvassa el előbb a [CONTRIBUTING.md](CONTRIBUTING.md) fájlt.

---

## Licenc

Ez a projekt MIT licenc alatt van - a részletekért lásd a [LICENSE](LICENSE) fájlt.

---

## Támogatás

- **Problémák**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Viták**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Készítve ❤️-vel <a href="https://github.com/mh3nj">A Te Neved</a>
</p>
