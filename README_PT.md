<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Gerenciador de Ativos Criativos Offline</strong>
</p>

<p align="center">
  <a href="#features">Funcionalidades</a> •
  <a href="#installation">Instalação</a> •
  <a href="#development">Desenvolvimento</a> •
  <a href="#architecture">Arquitetura</a> •
  <a href="#contributing">Contribuir</a> •
  <a href="#license">Licença</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versão">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licença">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plataforma">
</p>

---

## Sobre

Evoury é um poderoso gerenciador de ativos criativos offline, construído com Tauri, React e Rust. Projetado para profissionais criativos que precisam de acesso rápido e confiável aos seus ativos digitais sem comprometer o desempenho ou a privacidade.

### Por que Evoury?

- **Offline**: Seus ativos permanecem em sua máquina. Sem dependência de nuvem.
- **Ultra rápido**: Construído com Rust para desempenho que escala com sua biblioteca.
- **Arquitetura modular**: Mais de 40 crates especializados para máxima flexibilidade.
- **Interface bonita**: Interface moderna e responsiva construída com React e Tailwind CSS.

---

## Funcionalidades

### Motor Principal

- **Suporte multiformato**: Imagens, vídeos, modelos 3D, áudio, documentos e mais
- **Emparelhamento inteligente**: Agrupa automaticamente arquivos relacionados
- **Máquina de estados**: Rastreia ativos desde a descoberta até o arquivo
- **Arquitetura baseada em eventos**: Comunicação desacoplada via barramento de eventos

### Gerenciamento de Bibliotecas

- **Scanner avançado**: Modos de escaneamento completo, incremental, por pasta e em segundo plano
- **Observador do sistema de arquivos**: Sincronização em tempo real sem atualização manual
- **Pipeline de metadados**: Extração, normalização, validação e cache automáticos
- **Detecção de duplicatas**: SHA256, hash perceptual e baseado em metadados

### Pesquisa e Organização

- **Índice de pesquisa persistente**: Pesquisa de texto completo ultrarrápida com FTS5
- **Coleções inteligentes**: Coleções baseadas em regras com atualização automática
- **Linguagem de consulta avançada**: Filtros por tipo, tag, classificação, data, câmera e mais
- **Perfis de pesquisa**: Salve e alterne entre configurações de pesquisa

### Sistema de Espaço de Trabalho

- **Espaços de trabalho persistentes**: Lembra de todo o estado da sessão
- **Múltiplos espaços de trabalho**: Alterne entre diferentes contextos de projeto
- **Estações de trabalho**: Layouts pré-configurados com ferramentas, atalhos e temas
- **Painéis docáveis**: Motor de layout totalmente personalizável

### Saúde e Manutenção

- **Motor de saúde**: Verifica integridade do sistema de arquivos, banco de dados, cache e metadados
- **Reparo automático**: Reparo com um clique para problemas detectados
- **Recuperação de sessão**: Restaura o espaço de trabalho após desligamentos inesperados
- **Modo de suspensão**: Uso mínimo de recursos quando ocioso

---

## Capturas de Tela

<p align="center">
  <img src="public/images/main_dark.webp" alt="Interface Principal" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interface Principal - Visualização em Galeria</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Painel do Inspector" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Painel do Inspector - Detalhes do Ativo</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Interface de Pesquisa" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interface de Pesquisa Avançada</em>
</p>

---

## Instalação

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (última versão estável)
- [Node.js](https://nodejs.org/) (v18 ou superior)
- [pnpm](https://pnpm.io/) (v8 ou superior)

### Download

Baixe a última versão na página de [Releases](https://github.com/mh3nj/evoury/releases).

### Compilar a partir do Código Fonte

```bash
# Clonar o repositório
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instalar dependências
pnpm install

# Iniciar servidor de desenvolvimento
pnpm tauri dev

# Compilar para produção
pnpm tauri build
```

---

## Desenvolvimento

### Comandos Disponíveis

```bash
# Desenvolvimento
pnpm dev              # Iniciar servidor Vite
pnpm tauri dev        # Iniciar Tauri em modo desenvolvimento

# Compilação
pnpm build            # Compilar frontend
pnpm tauri build      # Compilar aplicação Tauri

# Testes
pnpm test             # Executar testes frontend
cargo test            # Executar testes Rust

# Lint
pnpm lint             # Executar ESLint
cargo clippy          # Executar Clippy

# Formatação
pnpm format           # Formatar código frontend
cargo fmt             # Formatar código Rust
```

---

## Stack Tecnológica

### Backend

- **Rust** - Linguagem de programação de sistemas
- **Tauri** - Framework de aplicação desktop
- **SQLite** - Banco de dados local
- **Crossbeam** - Primitivas de programação concorrente

### Frontend

- **React** - Biblioteca de UI
- **TypeScript** - JavaScript tipado
- **Tailwind CSS** - Framework CSS utilitário
- **Zustand** - Gerenciamento de estado
- **Vite** - Ferramenta de build e servidor de desenvolvimento

---

## Roadmap

Veja [ROADMAP.md](ROADMAP.md) para o roadmap detalhado do desenvolvimento.

---

## Contribuir

Contribuições são bem-vindas! Por favor, leia [CONTRIBUTING.md](CONTRIBUTING.md) primeiro.

---

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

## Suporte

- **Problemas**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discussões**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Feito com ❤️ por <a href="https://github.com/mh3nj">Seu Nome</a>
</p>
