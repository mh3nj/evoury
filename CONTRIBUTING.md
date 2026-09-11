# Contributing to Evoury

Thank you for your interest in contributing to Evoury! This document provides guidelines and information for contributors.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Reporting Bugs](#reporting-bugs)
- [Requesting Features](#requesting-features)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive experience for everyone. We pledge to act and interact in ways that contribute to an open, friendly, diverse, and healthy community.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or later)
- [pnpm](https://pnpm.io/) (v8 or later)
- Git

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:

```bash
git clone https://github.com/mh3nj/evoury.git
cd evoury
```

3. Add the upstream remote:

```bash
git remote add upstream https://github.com/mh3nj/evoury.git
```

---

## Development Setup

### Install Dependencies

```bash
pnpm install
```

### Start Development

```bash
# Start the development server
pnpm tauri dev
```

This will start both the Vite dev server and the Tauri application.

### Available Commands

```bash
# Development
pnpm dev              # Start Vite dev server only
pnpm tauri dev        # Start full Tauri development

# Building
pnpm build            # Build frontend
pnpm tauri build      # Build Tauri app

# Testing
pnpm test             # Run frontend tests
cargo test            # Run Rust tests

# Linting
pnpm lint             # Run ESLint
cargo clippy          # Run Clippy

# Formatting
pnpm format           # Format frontend code
cargo fmt             # Format Rust code
```

---

## How to Contribute

### Reporting Bugs

Before creating a bug report:

1. Check the [issue tracker](https://github.com/mh3nj/evoury/issues) to see if the problem has already been reported
2. If it hasn't, [create a new issue](https://github.com/mh3nj/evoury/issues/new) with:
   - A clear, descriptive title
   - A detailed description of the problem
   - Steps to reproduce the issue
   - Expected vs actual behavior
   - Environment information (OS, Rust version, Node version)

### Suggesting Features

Feature suggestions are welcome! To suggest a feature:

1. Check if the feature has already been suggested
2. [Create a new issue](https://github.com/mh3nj/evoury/issues/new) with:
   - A clear, descriptive title
   - A detailed description of the proposed feature
   - Use cases and examples
   - Any relevant mockups or designs

### Contributing Code

1. **Find an issue**: Look for issues labeled `good first issue` or `help wanted`
2. **Comment on the issue**: Let others know you're working on it
3. **Create a branch**: Create a feature branch from `main`
4. **Make changes**: Implement your changes following coding standards
5. **Test thoroughly**: Ensure your changes work correctly
6. **Submit a PR**: Create a pull request with a clear description

---

## Pull Request Process

### Before Submitting

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code follows project style guidelines
- [ ] Documentation is updated (if applicable)
- [ ] Commit messages are clear and descriptive

### PR Template

```markdown
## Description

Brief description of the changes

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Related Issues

Closes #123

## Testing

Describe the tests you ran to verify your changes

## Checklist

- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code where necessary
- [ ] I have updated documentation accordingly
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing unit tests pass locally
```

### Review Process

1. **Automated checks**: CI/CD will run tests and linting
2. **Code review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Merge**: Once approved, your PR will be merged

---

## Coding Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write documentation comments for public items
- Handle errors properly with `Result` types
- Use meaningful variable and function names

### TypeScript/React

- Follow the existing ESLint configuration
- Use TypeScript for type safety
- Write functional components with hooks
- Use meaningful component and variable names
- Keep components small and focused
- Write unit tests for complex logic

### General

- Keep code clean and readable
- Write self-documenting code
- Add comments for complex logic
- Follow the existing code style
- Don't repeat yourself (DRY principle)
- Single responsibility principle

---

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### Examples

```
feat(search): add advanced query language support

Implement a query parser that supports boolean operators,
field filters, and date ranges.

Closes #123
```

```
fix(preview): resolve memory leak in thumbnail cache

Fix issue where cached thumbnails were not being properly
released from memory when the cache was cleared.
```

---

## Reporting Bugs

### Bug Report Template

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '...'
3. Scroll down to '...'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
- OS: [e.g., Windows 11, macOS 14, Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Node version: [e.g., 18.19.0]
- Evoury version: [e.g., 0.1.0]

**Additional context**
Add any other context about the problem here.
```

---

## Requesting Features

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

---

## Questions?

If you have questions about contributing, feel free to:

- Open a [discussion](https://github.com/mh3nj/evoury/discussions)
- Reach out on Discord (link coming soon)

Thank you for contributing to Evoury!
