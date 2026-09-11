<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>离线优先的创意资产管理器</strong>
</p>

<p align="center">
  <a href="#features">功能特性</a> •
  <a href="#installation">安装指南</a> •
  <a href="#development">开发指南</a> •
  <a href="#architecture">架构设计</a> •
  <a href="#contributing">参与贡献</a> •
  <a href="#license">许可证</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="版本">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust 版本">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="许可证">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="平台">
</p>

---

## 关于

Evoury 是一款强大的、离线优先的创意资产管理器，基于 Tauri、React 和 Rust 构建。专为需要快速、可靠访问数字资产而不妥协性能或隐私的创意专业人士设计。

### 为什么选择 Evoury？

- **离线优先**：您的资产保留在本机上。无需云依赖。
- **极速性能**：使用 Rust 构建，性能随库规模扩展。
- **模块化架构**：40+ 个专用 crate，提供最大灵活性。
- **精美界面**：使用 React 和 Tailwind CSS 构建的现代化响应式界面。

---

## 功能特性

### 核心引擎

- **多格式支持**：图片、视频、3D 模型、音频、文档等
- **智能资产配对**：自动分组相关文件（如 `.blend`、`.fbx`、`.png`）
- **资产状态机**：跟踪资产从发现、验证、索引到归档的生命周期
- **事件驱动架构**：通过事件总线实现服务解耦通信

### 库管理

- **高级扫描器**：全量、增量、指定文件夹和后台扫描模式
- **文件系统监视器**：实时同步，无需手动刷新
- **元数据管道**：自动提取、规范化、验证和缓存元数据
- **重复检测**：SHA256、感知哈希和基于元数据的检测

### 搜索与组织

- **持久化搜索索引**：使用 FTS5 实现极速全文搜索
- **智能集合**：基于规则的自动更新集合
- **高级查询语言**：按类型、标签、评分、日期、相机等过滤
- **搜索配置**：保存和切换搜索配置

### 工作区系统

- **持久化工作区**：记住整个会话状态
- **多工作区**：在不同项目上下文间切换
- **工作站**：预配置的布局、工具、快捷键和主题
- **可停靠面板**：完全可自定义的布局引擎

### 健康与维护

- **健康引擎**：检查文件系统、数据库、缓存和元数据完整性
- **自动修复**：一键修复检测到的问题
- **会话恢复**：在意外关闭后恢复工作区
- **睡眠模式**：空闲时最小化资源使用

---

## 截图

<p align="center">
  <img src="public/images/main_dark.webp" alt="主界面" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>主界面 - 图库视图</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="检查器面板" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>检查器面板 - 资产详情</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="搜索界面" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>高级搜索界面</em>
</p>

---

## 安装

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install)（最新稳定版）
- [Node.js](https://nodejs.org/)（v18 或更高版本）
- [pnpm](https://pnpm.io/)（v8 或更高版本）

### 下载

从 [Releases](https://github.com/mh3nj/evoury/releases) 页面下载最新版本。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/mh3nj/evoury.git
cd evoury

# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

---

## 开发

### 可用命令

```bash
# 开发
pnpm dev              # 启动 Vite 开发服务器
pnpm tauri dev        # 启动 Tauri 开发模式

# 构建
pnpm build            # 构建前端
pnpm tauri build      # 构建生产版本

# 测试
pnpm test             # 运行前端测试
cargo test            # 运行 Rust 测试

# 代码检查
pnpm lint             # 运行 ESLint
cargo clippy          # 运行 Clippy

# 格式化
pnpm format           # 格式化前端代码
cargo fmt             # 格式化 Rust 代码
```

---

## 技术栈

### 后端

- **Rust** - 系统编程语言
- **Tauri** - 桌面应用框架
- **SQLite** - 本地数据库
- **Crossbeam** - 并发编程原语

### 前端

- **React** - UI 库
- **TypeScript** - 类型安全的 JavaScript
- **Tailwind CSS** - 实用优先的 CSS 框架
- **Zustand** - 状态管理
- **Vite** - 构建工具和开发服务器

---

## 路线图

详见 [ROADMAP.md](ROADMAP.md)。

---

## 参与贡献

欢迎贡献！请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

---

## 许可证

本项目使用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## 支持

- **问题反馈**：[GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **讨论区**：[GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  由 <a href="https://github.com/mh3nj">Mohsen Jafari</a> 用 ❤️ 制作
</p>
