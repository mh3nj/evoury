<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Izvanmrežni Kreativni Upravitelj Resursa</strong>
</p>

<p align="center">
  <a href="#features">Značajke</a> •
  <a href="#installation">Instalacija</a> •
  <a href="#development">Razvoj</a> •
  <a href="#architecture">Arhitektura</a> •
  <a href="#contributing">Doprinos</a> •
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

Evoury je moćan, izvanmrežni kreativni upravitelj resursa izgrađen pomoću Tauri, React i Rusta. Dizajniran za kreativne profesionalce koji trebaju brz i pouzdan pristup svojim digitalnim resursima bez kompromisa u performansi ili privatnosti.

### Zašto Evoury?

- **Izvanmrežni**: Vaši resursi ostaju na vašem računalu. Bez oblaka ovisnosti.
- **Munjevito brz**: Izgrađen s Rustom za performanse koje skaliraju s vašom bibliotekom.
- **Modularna arhitektura**: Više od 40 specijaliziranih crateova za maksimalnu fleksibilnost.
- **Predivno sučelje**: Moderno, responzivno sučelje izgrađeno s React i Tailwind CSS-om.

---

## Značajke

### Glavni Motor

- **Višeformatna podrška**: Slike, videi, 3D modeli, zvuk, dokumenti i više
- **Pametno sparivanje**: Automatski grupira povezane datoteke
- **Stanje resursa**: Prati resurse od otkrivanja do arhiviranja
- **Dogovđajno upravljana arhitektura**: Odvojena komunikacija putem sabirnice događaja

### Upravljanje Bibliotekama

- **Napredni skener**: Puno, inkrementalno, mapno specifično i pozadinsko skeniranje
- **Promatrač datotečnog sustava**: Sinhronizacija u stvarnom vremenu bez ručnog osvježavanja
- **Metapodatak cjevovod**: Automatsko izvlačenje, normalizacija, validacija i predmemoriranje
- **Otkrivanje duplikata**: SHA256, perceptualno heširanje i metapodatcima temeljeno

### Pretraživanje i Organizacija

- **Trajni indeks pretraživanja**: Munjevito brzo pretraživanje cijelog teksta s FTS5
- **Pametne kolekcije**: Pravilima temeljene automatski ažurirajuće kolekcije
- **Napredni jezik upita**: Filtri po tipu, oznaci, ocjeni, datumu, kameri i više
- **Profili pretraživanja**: Spremite i prebacite se između konfiguracija pretraživanja

### Sustav Radnog Prostora

- **Trajni radni prostori**: Pamti cijelo stanje sesije
- **Više radnih prostora**: Prebacite se između različitih konteksta projekata
- **Radne stanice**: Unaprijed konfigurirani rasporedi s alatima, prečacima i temama
- **Usidrivi paneli**: Potpuno prilagodljiv motor rasporeda

### Zdravlje i Održavanje

- **Zdravstveni motor**: Provjerava integritet datotečnog sustava, baze podataka, predmemorije i metapodataka
- **Automatski popravak**: Popravak jednim klikom za otkrivene probleme
- **Oporavak sesije**: Vraća radni prostor nakon neočekivanih gašenja
- **Način rada spavanja**: Minimalna upotreba resursa u neaktivnosti

---

## Screenshotovi

<p align="center">
  <img src="public/images/main_dark.webp" alt="Glavno Sučelje" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Glavno Sučelje - Prikaz Galerije</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Inspektorski Panel" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Inspektorski Panel - Detalji Resursa</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Sučelje Pretraživanja" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Napredno Sučelje Pretraživanja</em>
</p>

---

## Instalacija

### Preduvjeti

- [Rust](https://www.rust-lang.org/tools/install) (najnovija stabilna verzija)
- [Node.js](https://nodejs.org/) (v18 ili noviji)
- [pnpm](https://pnpm.io/) (v8 ili noviji)

### Preuzimanje

Preuzmite najnoviju verziju sa stranice [Releases](https://github.com/mh3nj/evoury/releases).

### Gradnja iz Izvora

```bash
# Klonirajte repozitorij
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instalirajte ovisnosti
pnpm install

# Pokrenite razvojni poslužitelj
pnpm tauri dev

# Gradite za produkciju
pnpm tauri build
```

---

## Razvoj

### Dostupne Naredbe

```bash
# Razvoj
pnpm dev              # Pokrenite Vite razvojni poslužitelj
pnpm tauri dev        # Pokrenite Tauri u razvojnom načinu

# Gradnja
pnpm build            # Gradite frontend
pnpm tauri build      # Gradite Tauri aplikaciju za produkciju

# Testovi
pnpm test             # Pokrenite frontend testove
cargo test            # Pokrenite Rust testove

# Lint
pnpm lint             # Pokrenite ESLint
cargo clippy          # Pokrenite Clippy

# Formatiranje
pnpm format           # Formatirajte frontend kod
cargo fmt             # Formatirajte Rust kod
```

---

## Tech Stack

### Backend

- **Rust** - Sistemski programski jezik
- **Tauri** - Framework desktop aplikacija
- **SQLite** - Lokalna baza podataka
- **Crossbeam** - Primitivi za konkurentnost

### Frontend

- **React** - UI biblioteka
- **TypeScript** - Tipovno siguran JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **Zustand** - Upravljanje stanjem
- **Vite** - Alat za gradnju i razvojni poslužitelj

---

## Karta puta

Za detaljnu kartu puta razvoja pogledajte [ROADMAP.md](ROADMAP.md).

---

## Doprinos

Doprinosi su dobrodošli! Molimo pročitajte prvo [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Licenca

Ovaj projekt je licenciran pod MIT licencom - pogledajte datoteku [LICENSE](LICENSE) za detalje.

---

## Podrška

- **Problemi**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Rasprave**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Napravljeno s ❤️ od <a href="https://github.com/mh3nj">Vaše Ime</a>
</p>
