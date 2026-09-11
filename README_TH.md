<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>ตัวจัดการสินทรัพย์สร้างสรรค์ออฟไลน์เป็นอันดับแรก</strong>
</p>

<p align="center">
  <a href="#features">คุณสมบัติ</a> •
  <a href="#installation">การติดตั้ง</a> •
  <a href="#development">การพัฒนา</a> •
  <a href="#architecture">สถาปัตยกรรม</a> •
  <a href="#contributing">การมีส่วนร่วม</a> •
  <a href="#license">ใบอนุญาต</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="เวอร์ชัน">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="ใบอนุญาต">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="แพลตฟอร์ม">
</p>

---

## เกี่ยวกับ

Evoury เป็นตัวจัดการสินทรัพย์สร้างสรรค์ออฟไลน์ที่ทรงพลัง สร้างด้วย Tauri, React และ Rust ออกแบบมาสำหรับนักสร้างสรรค์มืออาชีพที่ต้องการการเข้าถึงที่รวดเร็วและเชื่อถือได้ไปยังสินทรัพย์ดิจิทัลโดยไม่ต้องประนีประนอมกับประสิทธิภาพหรือความเป็นส่วนตัว

### ทำไม Evoury?

- **ออฟไลน์เป็นอันดับแรก**: สินทรัพย์ของคุณยังคงอยู่บนเครื่องของคุณ ไม่มีการพึ่งพาคลาวด์
- **เร็วเหมือนสายฟ้า**: สร้างด้วย Rust เพื่อประสิทธิภาพที่ปรับขนาดได้ตามขนาดห้องสมุดของคุณ
- **สถาปัตยกรรมแบบโมดูลาร์**: 40+ crate ที่เชี่ยวชาญสำหรับความยืดหยุ่นสูงสุด
- **UI ที่สวยงาม**: อินเทอร์เฟซที่ทันสมัยและตอบสนองได้ดีสร้างด้วย React และ Tailwind CSS

---

## คุณสมบัติ

### โมเตลหลัก

- **รองรับหลายรูปแบบ**: รูปภาพ, วิดีโอ, โมเดล 3D, เสียง, เอกสาร และอื่นๆ
- **การจับคู่อัจฉริยะ**: จัดกลุ่มไฟล์ที่เกี่ยวข้องโดยอัตโนมัติ
- **สถานะสินทรัพย์**: ติดตามสินทรัพย์จากการค้นพบจนถึงการเก็บถาวร
- **สถาปัตยกรรมที่ขับเคลื่อนด้วยเหตุการณ์**: การสื่อสารแบบแยกส่วนผ่าน buses เหตุการณ์

### การจัดการห้องสมุด

- **สแกนเนอร์ขั้นสูง**: โหมดสแกนเต็ม, ทีละขั้น, เฉพาะโฟลเดอร์ และพื้นหลัง
- **ตัวเฝ้าดูระบบไฟล์**: การซิงค์แบบเรียลไทม์โดยไม่ต้องรีเฟรชด้วยตนเอง
- **ท่อข้อมูลเมตา**: การแยก, การทำให้เป็นมาตรฐาน, การตรวจสอบ และการแคชอัตโนมัติ
- **การตรวจจับซ้ำ**: SHA256, hashing แบบรับรู้ และการตรวจจับที่ใช้ข้อมูลเมตา

### การค้นหาและจัดระเบียบ

- **ดัชนีการค้นหาถาวร**: การค้นหาข้อความทั้งหมดที่เร็วมากด้วย FTS5
- **คอลเลกชันอัจฉริยะ**: คอลเลกชันที่ใช้กฎและอัปเดตอัตโนมัติ
- **ภาษานิรุศกรรมขั้นสูง**: ตัวกรองตามประเภท, แท็ก, คะแนน, วันที่, กล้อง และอื่นๆ
- **โปรไฟล์การค้นหา**: บันทึกและสลับระหว่างการตั้งค่าการค้นหา

### ระบบพื้นที่ทำงาน

- **พื้นที่ทำงานถาวร**: จำสถานะเซสชันทั้งหมด
- **หลายพื้นที่ทำงาน**: สลับระหว่างบริบทโปรเจกต์ที่แตกต่างกัน
- **สถานีทำงาน**: เค้าโครงที่กำหนดค่าล่วงหน้าพร้อมเครื่องมือ, ทางลัด และธีม
- **แผงที่ทอด้วย**: โมเตลเค้าโครงที่ปรับแต่งได้อย่างสมบูรณ์

### สุขภาพและการบำรุงรักษา

- **โมเตลสุขภาพ**: ตรวจสอบความสมบูรณ์ของระบบไฟล์, ฐานข้อมูล, แคช และข้อมูลเมตา
- **การซ่อมอัตโนมัติ**: การซ่อมด้วยคลิกเดียวสำหรับปัญหาที่ตรวจพบ
- **การกู้คืนเซสชัน**: กู้คืนพื้นที่ทำงานหลังจากปิดเครื่องที่ไม่คาดคิด
- **โหมดนอนหลับ**: การใช้ทรัพยากรขั้นต่ำเมื่อว่าง

---

## ภาพหน้าจอ

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="อินเทอร์เฟซหลัก" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>อินเทอร์เฟซหลัก - มุมมองแกลเลอรี่</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="แผงตรวจสอบ" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>แผงตรวจสอบ - รายละเอียดสินทรัพย์</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="อินเทอร์เฟซการค้นหา" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>อินเทอร์เฟซการค้นหาขั้นสูง</em>
</p>

---

## การติดตั้ง

### ข้อกำหนดเบื้องต้น

- [Rust](https://www.rust-lang.org/tools/install) (เวอร์ชันเสถียรล่าสุด)
- [Node.js](https://nodejs.org/) (v18 หรือใหม่กว่า)
- [pnpm](https://pnpm.io/) (v8 หรือใหม่กว่า)

### การดาวน์โหลด

ดาวน์โหลดเวอร์ชันล่าสุดจากหน้า [Releases](https://github.com/mh3nj/evoury/releases)

### การสร้างจากซอร์สโค้ด

```bash
# โคลนรีโพซิทอรี
git clone https://github.com/mh3nj/evoury.git
cd evoury

# ติดตั้ง dependencies
pnpm install

# เริ่มต้นเซิร์ฟเวอร์พัฒนา
pnpm tauri dev

# สร้างสำหรับการผลิต
pnpm tauri build
```

---

## การพัฒนา

### คำสั่งที่มี

```bash
# การพัฒนา
pnpm dev              # เริ่มต้น Vite dev server
pnpm tauri dev        # เริ่มต้น Tauri ในโหมดพัฒนา

# การสร้าง
pnpm build            # สร้าง frontend
pnpm tauri build      # สร้าง Tauri app สำหรับการผลิต

# การทดสอบ
pnpm test             # รัน frontend tests
cargo test            # รัน Rust tests

# การตรวจสอบ
pnpm lint             # รัน ESLint
cargo clippy          # รัน Clippy

# การจัดรูปแบบ
pnpm format           # จัดรูปแบบ frontend code
cargo fmt             # จัดรูปแบบ Rust code
```

---

## Tech Stack

### แบ็กเอนด์

- **Rust** - ภาษาโปรแกรมมิ่งระบบ
- **Tauri** - โครงสร้างแอปพลิเคชันเดสก์ท็อป
- **SQLite** - ฐานข้อมูลท้องถิ่น
- **Crossbeam** - primitives การทำงานพร้อมกัน

### ฟรอนต์เอนด์

- **React** - คลัง UI
- **TypeScript** - JavaScript ที่ปลอดภัยตามประเภท
- **Tailwind CSS** - โครงสร้าง CSS แบบ utility-first
- **Zustand** - การจัดการสถานะ
- **Vite** - เครื่องมือสร้างและ dev server

---

## แผนที่ถนน

ดู [ROADMAP.md](ROADMAP.md) สำหรับแผนที่ถนนการพัฒนาโดยละเอียด

---

## การมีส่วนร่วม

การมีส่วนร่วมยินดีต้อนรับ! กรุณาอ่าน [CONTRIBUTING.md](CONTRIBUTING.md) ก่อน

---

## ใบอนุญาต

โครงการนี้ได้รับอนุญาตภายใต้ใบอนุญาต MIT - ดูไฟล์ [LICENSE](LICENSE) สำหรับรายละเอียด

---

## การสนับสนุน

- **ปัญหา**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **การอภิปราย**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  สร้างด้วย ❤️ โดย <a href="https://github.com/mh3nj">ชื่อของคุณ</a>
</p>
