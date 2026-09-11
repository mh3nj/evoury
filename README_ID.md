<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Pengelola Aset Kreatif Offline-Pertama</strong>
</p>

<p align="center">
  <a href="#features">Fitur</a> •
  <a href="#installation">Instalasi</a> •
  <a href="#development">Pengembangan</a> •
  <a href="#architecture">Arsitektur</a> •
  <a href="#contributing">Berkontribusi</a> •
  <a href="#license">Lisensi</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versi">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lisensi">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Tentang

Evoury adalah pengelola aset kreatif yang kuat, offline-pertama yang dibangun menggunakan Tauri, React dan Rust. Dirancang untuk profesional kreatif yang membutuhkan akses cepat dan andal ke aset digital mereka tanpa mengorbankan privasi atau kinerja.

### Mengapa Evoury?

- **Offline-pertama**: Aset Anda tetap di mesin Anda. Tergantung tanpa cloud.
- **Cepat seperti kilat**: Dibangun dengan Rust untuk kinerja yang dapat diskala dengan perpustakaan Anda.
- **Arsitektur modular**: Lebih dari 40 krate khusus untuk fleksibilitas maksimal.
- **UI cantik**: Antarmuka modern dan responsif dibangun dengan React dan Tailwind CSS.

---

## Fitur

### Mesin Inti

- **Dukungan multi-format**: Gambar, video, model 3D, audio, dokumen dan banyak lagi
- **Penjodohan cerdas**: Secara otomatis mengelompokkan file yang berkaitan
- **Mesin keadaan aset**: Melacak aset dari penemuan hingga pengarsipan
- **Arsitektur berbasis peristiwa**: Komunikasi terpisah melalui bus peristiwa

### Pengelolaan Perpustakaan

- **Pemindai canggih**: Mode pemindaian penuh, inkremental, spesifik folder dan latar belakang
- **Pengawas sistem file**: Sinkronisasi real-time tanpa pembaruan manual
- **Pipeline metadata**: Ekstraksi, normalisasi, validasi dan caching otomatis
- **Pendeteksian duplikat**: SHA256, hashing perseptual dan berbasis metadata

### Pencarian dan Organisasi

- **Indeks pencarian persisten**: Pencarian teks penuh ultra cepat dengan FTS5
- **Koleksi cerdas**: Koleksi berbasis aturan dengan pembaruan otomatis
- **Bahasa kueri canggih**: Filter berdasarkan jenis, tag, peringkat, tanggal, kamera dan banyak lagi
- **Profil pencarian**: Simpan dan beralih di antara konfigurasi pencarian

### Sistem Ruang Kerja

- **Ruang kerja persisten**: Mengingat seluruh keadaan sesi
- **Beberapa ruang kerja**: Beralih di antara konteks proyek yang berbeda
- **Stasiun kerja**: Tata letak yang telah dikonfigurasi dengan alat, pintasan dan tema
- **Panel yang dapat ditambatkan**: Mesin tata letak yang dapat disesuaikan sepenuhnya

### Kesehatan dan Pemeliharaan

- **Mesin kesehatan**: Memeriksa integritas sistem file, basis data, cache dan metadata
- **Perbaikan otomatis**: Perbaikan sekali klik untuk masalah yang terdeteksi
- **Pemulihan sesi**: Memulihkan ruang kerja setelah penutupan tak terduga
- **Mode tidur**: Penggunaan sumber minimum saat tidak aktif

---

## Tangkapan Layar

<p align="center">
  <img src="public/images/main_dark.webp" alt="Antarmuka Utama" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Antarmuka Utama - Tampilan Galeri</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Panel Inspektur" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel Inspektur - Detail Aset</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Antarmuka Pencarian" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Antarmuka Pencarian Lanjutan</em>
</p>

---

## Instalasi

### Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (versi stabil terbaru)
- [Node.js](https://nodejs.org/) (v18 atau lebih baru)
- [pnpm](https://pnpm.io/) (v8 atau lebih baru)

### Unduh

Unduh versi terbaru dari halaman [Releases](https://github.com/mh3nj/evoury/releases)

### Bangun dari Sumber

```bash
# Klon repositori
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instal dependensi
pnpm install

# Mulai server pengembangan
pnpm tauri dev

# Bangun untuk produksi
pnpm tauri build
```

---

## Pengembangan

### Perintah yang Tersedia

```bash
# Pengembangan
pnpm dev              # Mulai Vite dev server
pnpm tauri dev        # Mulai Tauri dalam mode pengembangan

# Pembangunan
pnpm build            # Bangun frontend
pnpm tauri build      # Bangun Tauri app untuk produksi

# Pengujian
pnpm test             # Jalankan pengujian frontend
cargo test            # Jalankan pengujian Rust

# Pemeriksaan
pnpm lint             # Jalankan ESLint
cargo clippy          # Jalankan Clippy

# Pemformatan
pnpm format           # Format kode frontend
cargo fmt             # Format kode Rust
```

---

## Tumpukan Teknologi

### Backend

- **Rust** - Bahasa pemrograman sistem
- **Tauri** - Kerangka kerja aplikasi desktop
- **SQLite** - Basis data lokal
- **Crossbeam** - Primitive konkurensi

### Frontend

- **React** - Pustaka UI
- **TypeScript** - JavaScript tipe-aman
- **Tailwind CSS** - Kerangka kerja CSS utility-first
- **Zustand** - Pengelolaan keadaan
- **Vite** - Alat bangun dan server pengembangan

---

## Peta Jalan

Lihat [ROADMAP.md](ROADMAP.md) untuk peta jalan pengembangan terperinci.

---

## Berkontribusi

Kontribusi disambut! Silakan baca [CONTRIBUTING.md](CONTRIBUTING.md) terlebih dahulu.

---

## Lisensi

Proyek ini dilisensikan di bawah Lisensi MIT - lihat file [LICENSE](LICENSE) untuk detail.

---

## Dukungan

- **Masalah**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Diskusi**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Dibuat dengan ❤️ oleh <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
