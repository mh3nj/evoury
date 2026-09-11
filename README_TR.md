<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Çevrimdışı Birinci Yaratıcı Varlık Yöneticisi</strong>
</p>

<p align="center">
  <a href="#features">Özellikler</a> •
  <a href="#installation">Kurulum</a> •
  <a href="#development">Geliştirme</a> •
  <a href="#architecture">Mimari</a> •
  <a href="#contributing">Katkıda Bulunma</a> •
  <a href="#license">Lisans</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Sürüm">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Lisans">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Platform">
</p>

---

## Hakkında

Evoury, Tauri, React ve Rust ile oluşturulmuş güçlü, çevrimdışı birinci yaratıcı varlık yöneticisidir. Performans veya gizlilikten ödün vermeden dijital varlıklarına hızlı ve güvenilir erişim ihtiyaci duyan yaratıcı profesyoneller için tasarlanmıştır.

### Neden Evoury?

- **Çevrimdışı birinci**: Varlıklarınız makinenizde kalır. Bulut bağımlılığı yoktur.
- **Şaşırtıcı hızlı**: Kütüphanenizle ölçeklenen performans için Rust ile oluşturulmuştur.
- **Modüler mimari**: Maksimum esneklik için 40+ uzmanlaşmış crate.
- **Güzel arayüz**: React ve Tailwind CSS ile oluşturulmuş modern, duyarlı arayüz.

---

## Özellikler

### Çekirdek Motor

- **Çoklu format desteği**: Görüntüler, videolar, 3D modeller, ses, belgeler ve daha fazlası
- **Akıllı eşleştirme**: İlişkili dosyaları otomatik olarak gruplandırır
- **Varlık durum makinesi**: Keşiften arşivlemeye kadar varlıkları takip eder
- **Olay odaklı mimari**: Olay otobüsü aracılığıyla bağlantı kopuk iletişimi

### Kütüphane Yönetimi

- **Gelişmiş tarayıcı**: Tam, artışçı, klasöre özel ve arka plan tarama modları
- **Dosya sistemi izleyicisi**: Manuel yenileme olmadan gerçek zamanlı senkronizasyon
- **Metadata hattı**: Otomatik çıkarma, normalize etme, doğrulama ve önbelleğe alma
- **Tekrar algılama**: SHA256, algısal hash ve metadata tabanlı

### Arama ve Organizasyon

- **Kalıcı arama indeksi**: FTS5 ile yıldırım hızında tam metin arama
- **Akıllı koleksiyonlar**: Kurallara dayalı otomatik güncellenen koleksiyonlar
- **Gelişmiş sorgu dili**: Tür, etiket, puan, tarih, kamera ve daha fazlasına göre filtreleme
- **Arama profilleri**: Arama yapılandırmaları arasında geçiş yapın

### Çalışma Alanı Sistemi

- **Kalıcı çalışma alanları**: Tüm oturum durumunu hatırlar
- **Çoklu çalışma alanları**: Farklı proje bağlamları arasında geçiş yapın
- **İstasyonlar**: Araçlar, kısayollar ve temalar ile önceden yapılandırılmış düzenler
- **Dockable paneller**: Tamamen özelleştirilebilir düzen motoru

### Sağlık ve Bakım

- **Sağlık motoru**: Dosya sistemi, veritabanı, önbellek ve metadata bütünlüğünü kontrol eder
- **Otomatik onarım**: Algılanan sorunlar için tek tıklamayla onarım
- **Oturum kurtarma**: Beklenmeyen kapatmalar sonra çalışma alanını geri yükler
- **Uyku modu**: Boşta minimum kaynak kullanımı

---

## Ekran Görüntüleri

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Ana Arayüz" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Ana Arayüz - Galeri Görünümü</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="İzleyici Paneli" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>İzleyici Paneli - Varlık Detayları</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Arama Arayüzü" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Gelişmiş Arama Arayüzü</em>
</p>

---

## Kurulum

### Ön Gereksinimler

- [Rust](https://www.rust-lang.org/tools/install) (en son kararlı sürüm)
- [Node.js](https://nodejs.org/) (v18 veya üzeri)
- [pnpm](https://pnpm.io/) (v8 veya üzeri)

### İndirme

En son sürümü [Releases](https://github.com/mh3nj/evoury/releases) sayfasından indirin.

### Kaynaktan Derleme

```bash
# Depoyu klonlayın
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Bağımlılıkları kurun
pnpm install

# Geliştirme sunucusunu başlatın
pnpm tauri dev

# Üretim için derleyin
pnpm tauri build
```

---

## Geliştirme

### Mevcut Komutlar

```bash
# Geliştirme
pnpm dev              # Vite geliştirme sunucusunu başlatın
pnpm tauri dev        # Tauri'yi geliştirme modunda başlatın

# Derleme
pnpm build            # Frontend'i derleyin
pnpm tauri build      # Tauri uygulamasını üretim için derleyin

# Testler
pnpm test             # Frontend testlerini çalıştırın
cargo test            # Rust testlerini çalıştırın

# Lint
pnpm lint             # ESLint'i çalıştırın
cargo clippy          # Clippy'yi çalıştırın

# Biçimlendirme
pnpm format           # Frontend kodunu biçimlendirin
cargo fmt             # Rust kodunu biçimlendirin
```

---

## Teknoloji Yığını

### Backend

- **Rust** - Sistem programlama dili
- **Tauri** - Masaüstü uygulama çerçevesi
- **SQLite** - Yerel veritabanı
- **Crossbeam** - Eşzamanlılık ilkelleri

### Frontend

- **React** - UI kütüphanesi
- **TypeScript** - Tür güvenli JavaScript
- **Tailwind CSS** - Yardımcı program CSS çerçevesi
- **Zustand** - Durum yönetimi
- **Vite** - Derleme aracı ve geliştirme sunucusu

---

## Yol Haritası

Ayrıntılı geliştirme yol haritası için [ROADMAP.md](ROADMAP.md) dosyasına bakın.

---

## Katkıda Bulunma

Katkılar hoştur! Lütfen önce [CONTRIBUTING.md](CONTRIBUTING.md) dosyasını okuyun.

---

## Lisans

Bu proje MIT lisansı altında lisanslanmıştır - ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.

---

## Destek

- **Sorunlar**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Tartışmalar**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  <a href="https://github.com/mh3nj">Adınız</a> tarafından ❤️ ile yapılmıştır
</p>
