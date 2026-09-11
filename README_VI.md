<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Quản lý tài sản sáng tạo Offline-trước</strong>
</p>

<p align="center">
  <a href="#features">Tính năng</a> •
  <a href="#installation">Cài đặt</a> •
  <a href="#development">Phát triển</a> •
  <a href="#architecture">Kiến trúc</a> •
  <a href="#contributing">Đóng góp</a> •
  <a href="#license">Giấy phép</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Phiên bản">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Giấy phép">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Nền tảng">
</p>

---

## Giới thiệu

Evoury là một quản lý tài sản sáng tạo mạnh mẽ, offline-trước được xây dựng với Tauri, React và Rust. Được thiết kế cho các chuyên gia sáng tạo cần truy cập nhanh chóng và đáng tin cậy vào tài sản kỹ thuật số mà không phải hy sinh hiệu suất hoặc quyền riêng tư.

### Tại sao Evoury?

- **Offline-trước**: Tài sản của bạn vẫn ở trên máy của bạn. Không phụ thuộc vào đám mây.
- **Nhanh như tia chớp**: Xây dựng với Rust cho hiệu suất có thể mở rộng với thư viện của bạn.
- **Kiến trúc mô-đun**: Hơn 40 crate chuyên biệt cho sự linh hoạt tối đa.
- **Giao diện đẹp**: Giao diện hiện đại, nhạy bén xây dựng với React và Tailwind CSS.

---

## Tính năng

### Hạt nhân chính

- **Hỗ trợ đa định dạng**: Hình ảnh, video, mô hình 3D, âm thanh, tài liệu và nhiều hơn nữa
- **Ghép nối thông minh**: Tự động nhóm các tệp liên quan
- **Máy trạng thái tài sản**: Theo dõi tài sản từ phát hiện đến lưu trữ
- **Kiến trúc sự kiện**: Giao tiếp được tách biệt thông qua bus sự kiện

### Quản lý Thư viện

- **Quét nâng cao**: Chế độ quét đầy toàn bộ, từng bước, cụ thể thư mục và nền
- **Theo dõi hệ thống tệp**: Đồng bộ hóa thời gian thực mà không cần làm mới thủ công
- **Pipeline siêu dữ liệu**: Trích xuất, chuẩn hóa, xác thực và bộ nhớ đệm tự động
- **Phát hiện trùng lặp**: SHA256, băm nhận thức và dựa trên siêu dữ liệu

### Tìm kiếm và Tổ chức

- **Chỉ mục tìm kiếm vĩnh viễn**: Tìm kiếm văn bản đầy đủ cực nhanh với FTS5
- **Bộ sưu tập thông minh**: Bộ sưu tập dựa trên quy tắc với cập nhật tự động
- **Ngôn ngữ truy vấn nâng cao**: Bộ lọc theo loại, thẻ, đánh giá, ngày, camera và nhiều hơn nữa
- **Hồ sơ tìm kiếm**: Lưu và chuyển đổi giữa các cấu hình tìm kiếm

### Hệ thống Không gian làm việc

- **Không gian làm việc vĩnh viễn**: Nhớ toàn bộ trạng thái phiên
- **Nhiều không gian làm việc**: Chuyển đổi giữa các ngữ cảnh dự án khác nhau
- **Trạm làm việc**: Bố cục được định cấu hình sẵn với công cụ, phím tắt và chủ đề
- **Bảng có thể neo**: Động cơ bố cục có thể tùy chỉnh hoàn toàn

### Sức khỏe và Bảo trì

- **Động cơ sức khỏe**: Kiểm tra tính toàn vẹn của hệ thống tệp, cơ sở dữ liệu, bộ nhớ đệm và siêu dữ liệu
- **Sửa chữa tự động**: Sửa chữa bằng một cú nhấp cho các sự cố được phát hiện
- **Khôi phục phiên**: Khôi phục không gian làm việc sau khi tắt bất ngờ
- **Chế độ ngủ**: Sử dụng tài nguyên tối thiểu khi nhàn rỗi

---

## Ảnh chụp màn hình

<p align="center">
  <img src="docs/images/screenshot-main.png" alt="Giao diện Chính" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Giao diện Chính - Chế độ xem Thư viện</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-inspector.png" alt="Bảng Kiểm tra" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Bảng Kiểm tra - Chi tiết Tài sản</em>
</p>

<p align="center">
  <img src="docs/images/screenshot-search.png" alt="Giao diện Tìm kiếm" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Giao diện Tìm kiếm Nâng cao</em>
</p>

---

## Cài đặt

### Yêu cầu

- [Rust](https://www.rust-lang.org/tools/install) (phiên bản ổn định mới nhất)
- [Node.js](https://nodejs.org/) (v18 trở lên)
- [pnpm](https://pnpm.io/) (v8 trở lên)

### Tải xuống

Tải xuống phiên bản mới nhất từ trang [Releases](https://github.com/mh3nj/evoury/releases)

### Xây dựng từ Nguồn

```bash
# Clone kho lưu trữ
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Cài đặt phụ thuộc
pnpm install

# Khởi động máy chủ phát triển
pnpm tauri dev

# Xây dựng cho sản xuất
pnpm tauri build
```

---

## Phát triển

### Lệnh có sẵn

```bash
# Phát triển
pnpm dev              # Khởi động Vite dev server
pnpm tauri dev        # Khởi động Tauri ở chế độ phát triển

# Xây dựng
pnpm build            # Xây dựng frontend
pnpm tauri build      # Xây dựng Tauri app cho sản xuất

# Kiểm tra
pnpm test             # Chạy kiểm tra frontend
cargo test            # Chạy kiểm tra Rust

# Kiểm tra mã
pnpm lint             # Chạy ESLint
cargo clippy          # Chạy Clippy

# Định dạng
pnpm format           # Định dạng mã frontend
cargo fmt             # Định dạng mã Rust
```

---

## Stack Công nghệ

### Backend

- **Rust** - Ngôn ngữ lập trình hệ thống
- **Tauri** - Framework ứng dụng desktop
- **SQLite** - Cơ sở dữ liệu cục bộ
- **Crossbeam** - Primitive song song

### Frontend

- **React** - Thư viện UI
- **TypeScript** - JavaScript an toàn kiểu
- **Tailwind CSS** - Framework CSS utility-first
- **Zustand** - Quản lý trạng thái
- **Vite** - Công cụ build và dev server

---

## Lộ trình

Xem [ROADMAP.md](ROADMAP.md) để biết lộ trình phát triển chi tiết.

---

## Đóng góp

Đóng góp được chào đón! Vui lòng đọc [CONTRIBUTING.md](CONTRIBUTING.md) trước.

---

## Giấy phép

Dự án này được cấp phép theo Giấy phép MIT - xem tệp [LICENSE](LICENSE) để biết chi tiết.

---

## Hỗ trợ

- **Vấn đề**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Thảo luận**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Được tạo với ❤️ bởi <a href="https://github.com/mh3nj">Tên của bạn</a>
</p>
