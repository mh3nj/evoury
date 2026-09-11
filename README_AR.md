<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>مدير الأصول الإبداعية دون اتصال</strong>
</p>

<p align="center">
  <a href="#features">الميزات</a> •
  <a href="#installation">التثبيت</a> •
  <a href="#development">التطوير</a> •
  <a href="#architecture">البنية</a> •
  <a href="#contributing">المساهمة</a> •
  <a href="#license">الترخيص</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="الإصدار">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="الترخيص">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="المنصة">
</p>

---

## حول

Evoury هو مدير أصول إبداعية قوي ودون اتصال، مبني باستخدام Tauri وReact وRust. مصمم للمحترفين الإبداعيين الذين يحتاجون إلى وصول سريع وموثوق إلى أصولهم الرقمية دون التنازل عن الأداء أو الخصوصية.

### لماذا Evoury؟

- **دون اتصال**: تبقى أصولك على جهازك. لا اعتماد على السحابة.
- **سريع للغاية**: مبني باستخدام Rust لأداء يتماشى مع مكتبتك.
- **بنية معمارية معيارية**: أكثر من 40 حزمة متخصصة لأقصى مرونة.
- **واجهة جميلة**: واجهة عصرية ومتجاوبة مبنية باستخدام React وTailwind CSS.

---

## الميزات

### المحرك الأساسي

- **دعم متعدد الصيغ**: صور، فيديو، نماذج ثلاثية الأبعاد، صوت، مستندات والمزيد
- **تثقيف ذكي**: يجمع تلقائيًا الملفات المرتبطة
- **آلة حالة الأصول**: تتبع الأصول من الاكتشاف إلى الأرشفة
- **بنية معمارية قائمة على الأحداث**: اتصال منفصل عبر ناقل الأحداث

### إدارة المكتبات

- **ماسح متقدم**: أوضاع مسح كامل، تدريجي، محدد للمجلد، وخلفية
- **مراقب نظام الملفات**: مزامنة في الوقت الفعلي بدون تحديث يدوي
- **خط أنابيب البيانات الوصفية**: استخراج وتطبيع وتحقق وتخزين مؤقت تلقائي
- **كشف التكرارات**: SHA256 والتجزئة الإدراكية وال	data-based

### البحث والتنظيم

- **فهرس بحث دائم**: بحث نصي كامل فائق السرعة مع FTS5
- **مجموعات ذكية**: مجموعات قائمة على القواعد مع تحديث تلقائي
- **لغة استعلام متقدمة**: فلاتر حسب النوع والعلامة والتقييم والتواريخ والكاميرا والمزيد
- **ملفات تعريف البحث**: احفظ وتنقل بين تكوينات البحث

### نظام مساحة العمل

- **مساحات عمل دائمة**: تتذكر حالة الجلسة بالكامل
- **مساحات عمل متعددة**: التنقل بين سياقات المشاريع المختلفة
- **محطات عمل**: تخطيطات مسبقة الإعداد مع الأدوات والاختصارات والسمات
- **لوحات قابلة للرسو**: محرك تخطيط قابل للتخصيص بالكامل

### الصحة والصيانة

- **محرك الصحة**: يتحقق من سلامة نظام الملفات وقاعدة البيانات والتخزين المؤقت والبيانات الوصفية
- **إصلاح تلقائي**: إصلاح بنقرة واحدة للمشاكل المكتشفة
- **استعادة الجلسة**: استعادة مساحة العمل بعد إيقاف التشغيل غير المتوقع
- **وضع السكون**: استخدام الحد الأدنى من الموارد أثناء الخمول

---

## لقطات الشاشة

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="الواجهة الرئيسية" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>الواجهة الرئيسية — عرض المعرض</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="لوحة الفاحص" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>لوحة الفاحص — تفاصيل الأصل</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="واجهة البحث" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>واجهة البحث المتقدم</em>
</p>

---

## التثبيت

### المتطلبات الأولية

- [Rust](https://www.rust-lang.org/tools/install) (أحدث إصدار مستقر)
- [Node.js](https://nodejs.org/) (v18 أو أحدث)
- [pnpm](https://pnpm.io/) (v8 أو أحدث)

### التنزيل

قم بتنزيل أحدث إصدار من صفحة [Releases](https://github.com/mh3nj/evoury/releases).

### البناء من المصدر

```bash
# استنساخ المستودع
git clone https://github.com/mh3nj/evoury.git
cd evoury

# تثبيت التبعيات
pnpm install

# بدء خادم التطوير
pnpm tauri dev

# بناء للإنتاج
pnpm tauri build
```

---

## التطوير

### الأوامر المتاحة

```bash
# التطوير
pnpm dev              # بدء خادم Vite
pnpm tauri dev        # بدء Tauri في وضع التطوير

# البناء
pnpm build            # بناء الواجهة الأمامية
pnpm tauri build      # بناء تطبيق Tauri للإنتاج

# الاختبارات
pnpm test             # تشغيل اختبارات الواجهة الأمامية
cargo test            # تشغيل اختبارات Rust

# الفحص
pnpm lint             # تشغيل ESLint
cargo clippy          # تشغيل Clippy

# التنسيق
pnpm format           # تنسيق كود الواجهة الأمامية
cargo fmt             # تنسيق كود Rust
```

---

## مجموعة التقنيات

### الخادم الخلفي

- **Rust** — لغة برمجة الأنظمة
- **Tauri** — إطار عمل تطبيقات سطح المكتب
- **SQLite** — قاعدة بيانات محلية
- **Crossbeam** — بدائل البرمجة المتزامنة

### الواجهة الأمامية

- **React** — مكتبة واجهة المستخدم
- **TypeScript** — JavaScript مُرقَّم
- **Tailwind CSS** — إطار CSS أداة
- **Zustand** — إدارة الحالة
- **Vite** — أداة بناء وخادم التطوير

---

## خارطة الطريق

انظر [ROADMAP.md](ROADMAP.md) لخارطة طريق التطوير التفصيلية.

---

## المساهمة

المساهمات مرحب بها! يرجى قراءة [CONTRIBUTING.md](CONTRIBUTING.md) أولاً.

---

## الترخيص

هذا المشروع مرخص بموجب ترخيص MIT — انظر ملف [LICENSE](LICENSE) للتفاصيل.

---

## الدعم

- **المشكلات**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **المناقشات**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  صنع بـ ❤️ بواسطة <a href="https://github.com/mh3nj">اسمك</a>
</p>
