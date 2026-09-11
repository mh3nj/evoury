<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-Prvi Kreativni Upravitelj Virov</strong>
</p>

<p align="center">
  <a href="#features">Funkcije</a> •
  <a href="#installation">Namestitev</a> •
  <a href="#development">Razvoj</a> •
  <a href="#architecture">Arhitektura</a> •
  <a href="#contributing">Prispevanje</a> •
  <a href="#license">Licenca</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Verzija">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licenca">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platforma">
</p>

---

## O Projektu

Evoury je zmogljiv, offline-prvi kreativni upravitelj virov, zgrajen s Tauri, React in Rust. Zasnovan za kreativne profesionalce, ki potrebujejo hiter in zanesljiv dostop do svojih digitalnih virov brez kompromisov pri zmogljivosti ali zasebnosti.

### Zakaj Evoury?

- **Offline-prvi**: Vaši viri ostanejo na vašem računalniku. Nobene oblaka ni odvisnost.
- **Bliskovito hiter**: Zgrajen z Rust za zmogljivost, ki se prilagaja vaši knjižnici.
- **Modularna arhitektura**: Več kot 40 specializiranih crate-ov za maksimalno prilagodljivost.
- **Lep vmesnik**: Moderen, odziven vmesnik zgrajen s React in Tailwind CSS.

---

## Funkcije

### Jedrni Motor

- **Večformatna podpora**: Slike, videi, 3D modeli, zvok, dokumenti in več
- **Pametno parjenje**: Samodejno združuje povezane datoteke
- **Stanje virov**: Sledi virom od odkritja do arhiviranja
- **Dogodkovno usmerjena arhitektura**: Ločena komunikacija prek dogodkovnega vodila

### Upravljanje Knjižnic

- **Napredni skener**: Polno, inkrementalno, mapno specifično in ozadje skeniranje
- **Opazovalnik datotečnega sistema**: Sinhronizacija v realnem času brez ročne osvežitve
- **Metapodatkovna cevovod**: Samodejno ekstrahiranje, normalizacija, validacija in predpomnjenje
- **Zaznavanje dvojnikov**: SHA256, zaznavno razprševanje in metapodatkovno

### Iskanje in Organizacija

- **Trajni indeks iskanja**: Bliskovito hitro polno besedilno iskanje s FTS5
- **Pametne zbirke**: Pravilno osnovane samodejno posodabljajoče zbirke
- **Napredni poizvedbeni jezik**: Filtri po tipu, oznaki, oceni, datumu, kameri in več
- **Profili iskanja**: Shrani in preklopi med konfiguracijami iskanja

### Sistem Delovnega Prostora

- **Trajni delovni prostori**: Zapomni si celotno stanje seje
- **Več delovnih prostorov**: Preklopi med različnimi konteksti projektov
- **Delovne postaje**: Vnaprej konfigurirane postavitve z orodji, bližnjicami in temami
- **Sidrijoče se plošče**: Popolnoma prilagodljiv motor postavitve

### Zdravje in Vzdrževanje

- **Zdravstveni motor**: Preverja celovitost datotečnega sistema, baze podatkov, predpomnilnika in metapodatkov
- **Samodejno popravilo**: En klik popravilo za zaznane težave
- **Obnovitev seje**: Obnovi delovni prostor po nepričakovanih izklopih
- **Spanje**: Minimalna poraba virov v mirovanju

---

## Posnetki Zaslona

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Glavni Vmesnik" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Glavni Vmesnik - Pogled Galerije</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Inšpektorska Plošča" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inšpektorska Plošča - Podrobnosti Vira</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Vmesnik Iskanja" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Napredni Vmesnik Iskanja</em>
</p>

---

## Namestitev

### Predpostavke

- [Rust](https://www.rust-lang.org/tools/install) (najnovejša stabilna verzija)
- [Node.js](https://nodejs.org/) (v18 ali novejši)
- [pnpm](https://pnpm.io/) (v8 ali novejši)

### Prenos

Prenesite najnovejšo verzijo s strani [Releases](https://github.com/mh3nj/evoury/releases).

### Gradnja iz Virov

```bash
# Klonirajte repozitorij
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Namestite odvisnosti
pnpm install

# Zaženite razvojni strežnik
pnpm tauri dev

# Zgradite za proizvodnjo
pnpm tauri build
```

---

## Razvoj

### Razpoložljivi Ukazi

```bash
# Razvoj
pnpm dev              # Zaženite Vite razvojni strežnik
pnpm tauri dev        # Zaženite Tauri v razvojnem načinu

# Gradnja
pnpm build            # Zgradite frontend
pnpm tauri build      # Zgradite Tauri aplikacijo za proizvodnjo

# Testi
pnpm test             # Zaženite frontend teste
cargo test            # Zaženite Rust teste

# Lint
pnpm lint             # Zaženite ESLint
cargo clippy          # Zaženite Clippy

# Oblikovanje
pnpm format           # Oblikujte frontend kodo
cargo fmt             # Oblikujte Rust kodo
```

---

## Tech Stack

### Backend

- **Rust** - Sistemski programski jezik
- **Tauri** - Framework namiznih aplikacij
- **SQLite** - Lokalna baza podatkov
- **Crossbeam** - Primitivi za sočasnost

### Frontend

- **React** - UI knjižnica
- **TypeScript** - Tipovno varen JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Upravljanje stanja
- **Vite** - Orodje za gradnjo in razvojni strežnik

---

## Zemljevid Poti

Za podroben zemljevid razvoja glejte [ROADMAP.md](ROADMAP.md).

---

## Prispevanje

Prispevki so dobrodošli! Prosimo, preberite najprej [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenca

Ta projekt je licenciran pod MIT licenco - za podrobnosti glejte datoteko [LICENSE](LICENSE).

---

## Podpora

- **Težave**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Razprave**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Narejeno z ❤️ od <a href="https://github.com/mh3nj">Vaše Ime</a>
</p>
