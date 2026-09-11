<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Offline-ensinnäinen Luova Asset Manager</strong>
</p>

<p align="center">
  <a href="#features">Ominaisuudet</a> •
  <a href="#installation">Asennus</a> •
  <a href="#development">Kehitys</a> •
  <a href="#architecture">Arkkitehtuuri</a> •
  <a href="#contributing">Osallistuminen</a> •
  <a href="#license">Lisenssi</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versio">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lisenssi">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Alusta">
</p>

---

## Tietoa

Evoury on tehokas, offline-ensinnäinen luova asset manager, joka on rakennettu Tauri:lla, Reactilla ja Rustilla. Suunniteltu luoville ammattilaisille, jotka tarvitsevat nopeaa ja luotettavaa pääsyä digitaalisiin omaisuuseriinsa tinkimättä suorituskyvystä tai yksityisyydestä.

### Miksi Evoury?

- **Offline-ensinnäinen**: Omakkuutesi pysyvät koneellasi. Ei pilviriippuvuutta.
- **Salaman nopea**: Rustilla rakennettu suorituskyky, joka skaalautuu kirjastosi mukana.
- **Moduulinen arkkitehtuuri**: Yli 40 erikoistunutta cratea maksimaalista joustavuutta varten.
- **Kaunis käyttöliittymä**: Nykyaikainen, reagoiva käyttöliittymä rakennettu Reactilla ja Tailwind CSS:llä.

---

## Ominaisuudet

### Ydinmoottori

- **Monimuoto-tuki**: Kuvat, videot, 3D-mallit, ääni, asiakirjat ja lisää
- **Älykäs париngi**: Ryhmittää automaattisesti toisiaan vastaavat tiedostot
- **Asset tilakone**: Seuraa omakkuuksia löytymisestä arkistointiin
- **Tapahtumapohjainen arkkitehtuuri**: Irrotettu viestintä tapahtumabussilla

### Kirjastohallinta

- **Edistynyt skanneri**: Täys-, inkrementteli, kansio-erittelevä ja taustaskannus
- **Tiedostojärjestelmän tarkkailija**: Reaaliaikainen synkronointi ilman manuaalista päivitystä
- **Metatietoputki**: Automaattinen poiminta, normalisointi, validointi ja välimuisti
- **Kaksoiskappaleten tunnistus**: SHA256, aistivu hash ja metatietopohjainen

### Haku ja organisointi

- **Pysyvä hakuindeksi**: Salaman nopea täystekstihaku FTS5:llä
- **Älykäs kokoelmat**: Sääntöpohjaiset automaattisesti päivittyvät kokoelmat
- **Edistynyt kyselykieli**: Suodattimet tyypin, tunnuksen, arvostelun, päivän, kameran mukaan
- **Hakuprofiilit**: Tallenna ja vaihda hakukokoonpanojen välillä

### Työtilajärjestelmä

- **Pysyvät työtilat**: Muistaa koko istunnon tilan
- **Useat työtilat**: Vaihda eri projektikontekstien välillä
- **Työasemat**: Ennalta määritetyt asettelut työkaluilla, pikanäppäimillä ja teemoilla
- **Telakoitavat paneelit**: Täysin muokattavissa oleva asettelumoottori

### Terveys ja ylläpito

- **Terveysmoottori**: Tarkistaa tiedostojärjestelmän, tietokannan, välimuistin ja metatietojen eheyden
- **Automaattinen korjaus**: Yhden napsautuksen korjaus havaittuihin ongelmiin
- **Istunnon palautus**: Palauttaa työtilan odottamattomien sammutusten jälkeen
- **Unitila**: Minimi resurssien käyttö toimettomana

---

## Kuvakaappaukset

<p align="center">
  <img src="public/images/main_dark.webp" alt="Pääkäyttöliittymä" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Pääkäyttöliittymä - Gallerianäkymä</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Tarkastajapaneeli" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Tarkastajapaneeli - Omakkuuden tiedot</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Hakukäyttöliittymä" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Edistynyt hakukäyttöliittymä</em>
</p>

---

## Asennus

### Ennakkovaatimukset

- [Rust](https://www.rust-lang.org/tools/install) (uusin vakaa versio)
- [Node.js](https://nodejs.org/) (v18 tai uudempi)
- [pnpm](https://pnpm.io/) (v8 tai uudempi)

### Lataaminen

Lataa uusin versio [Releases](https://github.com/mh3nj/evoury/releases) sivulta.

### Kääntäminen lähteestä

```bash
# Kloonaa repo
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Asenna riippuvuudet
pnpm install

# Käynnistä kehityspalvelin
pnpm tauri dev

# Käännä tuotantoon
pnpm tauri build
```

---

## Kehitys

### Käytettävissä olevat komennot

```bash
# Kehitys
pnpm dev              # Käynnistä Vite kehityspalvelin
pnpm tauri dev        # Käynnistä Tauri kehitystilassa

# Kääntäminen
pnpm build            # Käännä frontend
pnpm tauri build      # Käännä Tauri sovellus tuotantoon

# Testit
pnpm test             # Suorita frontend testit
cargo test            # Suorita Rust testit

# Lint
pnpm lint             # Suorita ESLint
cargo clippy          # Suorita Clippy

# Muotoilu
pnpm format           # Muotoile frontend koodi
cargo fmt             # Muotoile Rust koodi
```

---

## Teknologiastakla

### Tausta

- **Rust** - Järjestelmäohjelmointikieli
- **Tauri** - Työpöytäsovelluskehys
- **SQLite** - Paikallinen tietokanta
- **Crossbeam** - Rinnakkaisuusprimitiivit

### Käyttöliittymä

- **React** - UI-kirjasto
- **TypeScript** - Tyyppiturvallinen JavaScript
- **Tailwind CSS** - Hyöty ensin CSS-kehykset
- **Zustand** - Tilanhallinta
- **Vite** - Kääntötyökalu ja kehityspalvelin

---

## Kartta

Katso [ROADMAP.md](#) yksityiskohtaisesta kehityskartasta.

---

## Osallistuminen

Osalluminen on tervetullut! Lue ensin [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Lisenssi

Tämä projekti on lisensoitu MIT-lisenssillä - katso yksityiskohdat tiedostosta [LICENSE](LICENSE).

---

## Tuki

- **Ongelmat**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Keskustelut**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Tehty ❤️:llä <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
