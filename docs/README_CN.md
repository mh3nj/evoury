# Evoury 项目结构

本目录包含 Evoury 项目的完整结构。

## 目录说明

```
evoury/
├── .github/              # GitHub 配置文件
│   ├── ISSUE_TEMPLATE/   # Issue 模板
│   ├── workflows/        # GitHub Actions 工作流
│   └── PULL_REQUEST_TEMPLATE.md
├── crates/               # Rust crates（后端模块）
│   ├── atlas-core/       # 核心领域模型
│   ├── atlas-events/     # 事件总线系统
│   ├── atlas-scanner/    # 库扫描
│   └── ...               # 40+ 专用 crates
├── src/                  # React 前端
│   ├── components/       # UI 组件
│   ├── hooks/            # React hooks
│   ├── stores/           # 状态管理
│   ├── pages/            # 页面组件
│   └── styles/           # 全局样式
├── src-tauri/           # Tauri 配置
├── public/              # 静态资源
├── docs/                # 文档
├── README.md            # 项目说明
├── CONTRIBUTING.md      # 贡献指南
├── ROADMAP.md           # 开发路线图
├── CHANGELOG.md         # 更新日志
└── LICENSE              # 许可证
```

## 快速开始

### 环境要求

- [Rust](https://www.rust-lang.org/tools/install)（最新稳定版）
- [Node.js](https://nodejs.org/)（v18 或更高版本）
- [pnpm](https://pnpm.io/)（v8 或更高版本）

### 安装依赖

```bash
pnpm install
```

### 启动开发

```bash
# 启动开发服务器
pnpm tauri dev
```

## 参与贡献

请阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 了解如何参与贡献。

## 许可证

本项目使用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。
