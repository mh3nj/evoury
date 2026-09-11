<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Pengurus Aset Kreatif Offline-Pertama</strong>
</p>

<p align="center">
  <a href="#features">Ciri-ciri</a> •
  <a href="#installation">Pemasangan</a> •
  <a href="#development">Pembangunan</a> •
  <a href="#architecture">Seni Bina</a> •
  <a href="#contributing">Menyumbang</a> •
  <a href="#license">Lesen</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versi">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lesen">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Tentang

Evoury adalah pengurus aset kreatif yang berkuasa, offline-pertama dibina menggunakan Tauri, React dan Rust. Direka untuk profesional kreatif yang memerlukan akses pantas dan boleh dipercayai kepada aset digital mereka tanpa berkompromi prestasi atau privasi.

### Mengapa Evoury?

- **Offline-pertama**: Aset anda kekal pada mesin anda. Tiada kebergantungan awan.
- **Pantas seperti kilat**: Dibina dengan Rust untuk prestasi yang berskala dengan perpustakaan anda.
- **Seni bina modular**: Lebih 40 krate khusus untuk fleksibiliti maksimum.
- **UI cantik**: Antara muka moden, responsif dibina dengan React dan Tailwind CSS.

---

## Ciri-ciri

### Enjin Teras

- **Sokongan berbilang format**: Imej, video, model 3D, audio, dokumen dan banyak lagi
- **Pasangan pintar**: Secara automatik mengumpul fail yang berkaitan
- **Mesin keadaan aset**: Menjejaki aset dari penemuan hingga arkib
- **Seni bina berasaskan peristiwa**: Komunikasi terpisah melalui bas peristiwa

### Pengurusan Perpustakaan

- **Imbasan lanjutan**: Mod imbasan penuh, berperingkat, spesifik folder dan latar belakang
- **Pemerhati sistem fail**: Penyegerakan masa nyata tanpa pembaharuan manual
- **Paip meta data**: Ekstrak, normalkan, sahkan dan cache automatik
- **Pengesanan pendua**: SHA256, hashing persepsi dan berasaskan meta data

### Pencarian dan Organisasi

- **Indeks carian berterusan**: Carian teks penuh ultra pantas dengan FTS5
- **Koleksi pintar**: Koleksi berasaskan peraturan dengan pembaharuan automatik
- **Bahasa pertanyaan lanjutan**: Penapis mengikut jenis, tag, penilaian, tarikh, kamera dan banyak lagi
- **Profil carian**: Simpan dan tukar antara konfigurasi carian

### Sistem Ruang Kerja

- **Ruang kerja berterusan**: Mengingati keseluruhan keadaan sesi
- **Berbilang ruang kerja**: Tukar antara konteks projek yang berbeza
- **Stesen kerja**: Tata letak yang telah dikonfigurasi dengan alat, pintasan dan tema
- **Panel boleh labuh**: Enjin tata letak yang boleh dilaraskan sepenuhnya

### Kesihatan dan Penyelenggaraan

- **Enjin kesihatan**: Menyemak integriti sistem fail, pangkalan data, cache dan meta data
- **Pembaikan automatik**: Pembaikan dengan satu klik untuk masalah yang dikesan
- **Pemulihan sesi**: Memulihkan ruang kerja selepas penutupan yang tidak dijangka
- **Mod tidur**: Penggunaan sumber minimum semasa melahu

---

## Tangkapan Skrin

<p align="center">
  <img src="public/images/main_dark.webp" alt="Antara Muka Utama" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Antara Muka Utama - Paparan Galeri</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Panel Pemeriksa" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel Pemeriksa - Butiran Aset</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Antara Muka Carian" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Antara Muka Carian Lanjutan</em>
</p>

---

## Pemasangan

### Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (versi stabil terkini)
- [Node.js](https://nodejs.org/) (v18 atau lebih baru)
- [pnpm](https://pnpm.io/) (v8 atau lebih baru)

### Muat Turun

Muat turun versi terkini dari halaman [Releases](https://github.com/mh3nj/evoury/releases)

### Bina dari Sumber

```bash
# Klon repositori
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Pasang kebergantungan
pnpm install

# Mulakan pelayar pembangunan
pnpm tauri dev

# Bina untuk pengeluaran
pnpm tauri build
```

---

## Pembangunan

### Arahan yang Tersedia

```bash
# Pembangunan
pnpm dev              # Mulakan Vite dev server
pnpm tauri dev        # Mulakan Tauri dalam mod pembangunan

# Pembinaan
pnpm build            # Bina frontend
pnpm tauri build      # Bina Tauri app untuk pengeluaran

# Ujian
pnpm test             # Jalankan ujian frontend
cargo test            # Jalankan ujian Rust

# Lint
pnpm lint             # Jalankan ESLint
cargo clippy          # Jalankan Clippy

# Pemformatan
pnpm format           # Format kod frontend
cargo fmt             # Format kod Rust
```

---

## Tumpukan Teknologi

### Backend

- **Rust** - Bahasa pengaturcaraan sistem
- **Tauri** - Kerangka kerja aplikasi desktop
- **SQLite** - Pangkalan data tempatan
- **Crossbeam** - Primitive serentak

### Frontend

- **React** - Pustaka UI
- **TypeScript** - JavaScript selamat jenis
- **Tailwind CSS** - Kerangka kerja CSS utility-first
- **Zustand** - Pengurusan keadaan
- **Vite** - Alat bina dan pelayar pembangunan

---

## Peta Jalan

Lihat [ROADMAP.md](ROADMAP.md) untuk peta jalan pembangunan terperinci.

---

## Menyumbang

Sumbangan dialu-alukan! Sila baca [CONTRIBUTING.md](CONTRIBUTING.md) terlebih dahulu.

---

## Lesen

Projek ini dilisensikan di bawah Lesen MIT - lihat fail [LICENSE](LICENSE) untuk butiran.

---

## Sokongan

- **Masalah**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Perbincangan**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Dibina dengan ❤️ oleh <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
