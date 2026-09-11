<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>مدیر دارایی خلاق آفلاین-اول</strong>
</p>

<p align="center">
  <a href="#features">ویژگی‌ها</a> •
  <a href="#installation">نصب</a> •
  <a href="#development">توسعه</a> •
  <a href="#architecture">معماری</a> •
  <a href="#contributing">مشارکت</a> •
  <a href="#license">مجوز</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="نسخه">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="مجوز">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="پلتفرم">
</p>

---

## درباره

Evoury یک مدیر دارایی خلاق قدرتمند و آفلاین-اول است که با Tauri، React و Rust ساخته شده است. برای حرفه‌ای‌های خلاق طراحی شده است که به دسترسی سریع و قابل اعتماد به دارایی‌های دیجیتال خود نیاز دارند بدون اینکه در عملکرد یا حریم خصوصی خود سازش کنند.

### چرا Evoury?

- **آفلاین-اول**: دارایی‌های شما روی دستگاه شما باقی می‌مانند. بدون وابستگی به ابر.
- **سریع مانند برق**: با Rust ساخته شده است برای عملکردی که با کتابخانه شما مقیاس پذیر است.
- **معماری ماژولار**: بیش از 40 crate تخصصی برای انعطاف پذیری حداکثری.
- **رابط زیبا**: رابط مدرن و واکنش‌گرا ساخته شده با React و Tailwind CSS.

---

## ویژگی‌ها

### موتور اصلی

- **پشتیبانی چند فرمت**: تصاویر، ویدیوها، مدل‌های 3D، صدا، اسناد و موارد دیگر
- **جفت‌سازی هوشمند**: فایل‌های مرتبط را به صورت خودکار گروه‌بندی می‌کند
- **ماشین حالت دارایی**: دارایی‌ها را از کشف تا بایگانی ردیابی می‌کند
- **معماری رویدادمحور**: ارتباط جداسازی شده از طریق اتوبوس رویداد

### مدیریت کتابخانه

- **اسکنر پیشرفته**: حالت‌های اسکن کامل، افزایشی، خاص پوشه و پس‌زمینه
- **ناظر سیستم فایل**: همگام‌سازی در زمان واقعی بدون به‌روزرسانی دستی
- **خط لوله متاداتا**: استخراج، عادی‌سازی، اعتبارسنجی و کش خودکار
- **تشخیص تکرار**: SHA256، هش ادراکی و مبتنی بر متاداتا

### جستجو و سازماندهی

- **ایندکس جستجوی پایدار**: جستجوی متن کامل فوق‌العاده سریع با FTS5
- **مجموعه‌های هوشمند**: مجموعه‌های مبتنی بر قانون با به‌روزرسانی خودکار
- **زبان پیشرفته پرس و جو**: فیلترها بر اساس نوع، برچسب، امتیاز، تاریخ، دوربین و موارد دیگر
- **پروفایل‌های جستجو**: ذخیره و جابجایی بین پیکربندی‌های جستجو

### سیستم فضای کاری

- **فضاهای کاری پایدار**: کل حالت جلسه را به یاد می‌آورد
- **فضاهای کاری متعدد**: بین زمینه‌های مختلف پروژه جابجا شوید
- **ایستگاه‌های کاری**: طرح‌بندی‌های از پیش پیکربندی شده با ابزارها، میانبرها و موضوعات
- **پنل‌های قابل لنگرگذاری**: موتور طرح‌بندی کاملاً قابل تنظیم

### سلامت و نگهداری

- **موتور سلامت**: یکپارچگی سیستم فایل، پایگاه داده، کش و متاداتا را بررسی می‌کند
- **تعمیر خودکار**: تعمیر با یک کلیک برای مشکلات شناسایی شده
- **بازیابی جلسه**: فضای کاری را پس از خاموشی‌های غیرمنتظره بازیابی می‌کند
- **حالت خواب**: استفاده حداقلی از منابع در حالت بیکاری

---

## اسکرین‌شات‌ها

<p align="center">
  <img src="public/images/main_dark.webp" alt="رابط اصلی" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>رابط اصلی - نمای گالری</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="پنل بازرس" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>پنل بازرس - جزئیات دارایی</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="رابط جستجو" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>رابط جستجوی پیشرفته</em>
</p>

---

## نصب

### پیش‌نیازها

- [Rust](https://www.rust-lang.org/tools/install) (آخرین نسخه پایدار)
- [Node.js](https://nodejs.org/) (v18 یا بالاتر)
- [pnpm](https://pnpm.io/) (v8 یا بالاتر)

### دانلود

آخرین نسخه را از صفحه [Releases](https://github.com/mh3nj/evoury/releases) دانلود کنید.

### ساخت از منبع

```bash
# مخزن را کلون کنید
git clone https://github.com/mh3nj/evoury.git
cd evoury

# وابستگی‌ها را نصب کنید
pnpm install

# سرور توسعه را شروع کنید
pnpm tauri dev

# برای تولید بسازید
pnpm tauri build
```

---

## توسعه

### دستورات در دسترس

```bash
# توسعه
pnpm dev              # سرور Vite را شروع کنید
pnpm tauri dev        # Tauri را در حالت توسعه شروع کنید

# ساخت
pnpm build            # فرانت‌اند را بسازید
pnpm tauri build      # اپلیکیشن Tauri را برای تولید بسازید

# تست‌ها
pnpm test             # تست‌های فرانت‌اند را اجرا کنید
cargo test            # تست‌های Rust را اجرا کنید

# لینت
pnpm lint             # ESLint را اجرا کنید
cargo clippy          # Clippy را اجرا کنید

# قالب‌بندی
pnpm format           # کد فرانت‌اند را قالب‌بندی کنید
cargo fmt             # کد Rust را قالب‌بندی کنید
```

---

## استک فناوری

### بک‌اند

- **Rust** - زبان برنامه‌نویسی سیستم
- **Tauri** - فریم‌ورک اپلیکیشن دسکتاپ
- **SQLite** - پایگاه داده محلی
- **Crossbeam** - primitives همزمانی

### فرانت‌اند

- **React** - کتابخانه UI
- **TypeScript** - JavaScript نوع ایمن
- **Tailwind CSS** - فریم‌ورک CSS ابزارمحور
- **Zustand** - مدیریت حالت
- **Vite** - ابزار ساخت و سرور توسعه

---

## نقشه راه

برای نقشه راه دقیق توسعه [ROADMAP.md](ROADMAP.md) را ببینید.

---

## مشارکت

مشارکت‌ها خوش آمدید! لطفاً ابتدا [CONTRIBUTING.md](CONTRIBUTING.md) را بخوانید.

---

## مجوز

این پروژه تحت مجوز MIT مجوز دارد - برای جزئیات فایل [LICENSE](LICENSE) را ببینید.

---

## پشتیبانی

- **مشکلات**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **بحث‌ها**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  ساخته شده با ❤️ توسط <a href="https://github.com/mh3nj">نام شما</a>
</p>
