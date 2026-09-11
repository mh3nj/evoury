<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>Administrador de Activos Creativos sin Conexión</strong>
</p>

<p align="center">
  <a href="#features">Características</a> •
  <a href="#installation">Instalación</a> •
  <a href="#development">Desarrollo</a> •
  <a href="#architecture">Arquitectura</a> •
  <a href="#contributing">Contribuir</a> •
  <a href="#license">Licencia</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Versión">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="Licencia">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="Plataforma">
</p>

---

## Acerca de

Evoury es un administrador de activos creativos potente y sin conexión, construido con Tauri, React y Rust. Diseñado para profesionales creativos que necesitan acceso rápido y confiable a sus activos digitales sin comprometer el rendimiento o la privacidad.

### ¿Por qué Evoury?

- **Sin conexión**: Sus activos permanecen en su máquina. Sin dependencia de la nube.
- **Ultra rápido**: Construido con Rust para un rendimiento que escala con su biblioteca.
- **Arquitectura modular**: Más de 40 crates especializados para máxima flexibilidad.
- **Hermosa interfaz**: Interfaz moderna y receptiva construida con React y Tailwind CSS.

---

## Características

### Motor Principal

- **Soporte multiformato**: Imágenes, videos, modelos 3D, audio, documentos y más
- **Emparejamiento inteligente**: Agrupa automáticamente archivos relacionados
- **Máquina de estados**: Rastrea activos desde el descubrimiento hasta el archivo
- **Arquitectura basada en eventos**: Comunicación desacoplada mediante bus de eventos

### Gestión de Bibliotecas

- **Escáner avanzado**: Modos de escaneo completo, incremental, por carpeta y en segundo plano
- **Observador del sistema de archivos**: Sincronización en tiempo real sin actualización manual
- **Pipeline de metadatos**: Extracción, normalización, validación y caché automática
- **Detección de duplicados**: SHA256, hash perceptivo y basado en metadatos

### Búsqueda y Organización

- **Índice de búsqueda persistente**: Búsqueda de texto completo ultrarrápida con FTS5
- **Colecciones inteligentes**: Colecciones basadas en reglas con actualización automática
- **Lenguaje de consulta avanzado**: Filtros por tipo, etiqueta, calificación, fecha, cámara y más
- **Perfiles de búsqueda**: Guarde y cambie entre configuraciones de búsqueda

### Sistema de Espacios de Trabajo

- **Espacios de trabajo persistentes**: Recuerda todo el estado de la sesión
- **Múltiples espacios de trabajo**: Cambie entre diferentes contextos de proyecto
- **Estaciones de trabajo**: Diseños preconfigurados con herramientas, atajos y temas
- **Paneles anclables**: Motor de diseño completamente personalizable

### Salud y Mantenimiento

- **Motor de salud**: Verifica la integridad del sistema de archivos, base de datos, caché y metadatos
- **Reparación automática**: Reparación con un clic para problemas detectados
- **Recuperación de sesión**: Restaura el espacio de trabajo después de apagados inesperados
- **Modo sueño**: Uso mínimo de recursos cuando está inactivo

---

## Capturas de Pantalla

<p align="center">
  <img src="public/images/main_dark.webp" alt="Interfaz Principal" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfaz Principal - Vista de Galería</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="Panel de Inspección" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Panel de Inspección - Detalles del Activo</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="Interfaz de Búsqueda" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>Interfaz de Búsqueda Avanzada</em>
</p>

---

## Instalación

### Requisitos Previos

- [Rust](https://www.rust-lang.org/tools/install) (última versión estable)
- [Node.js](https://nodejs.org/) (v18 o posterior)
- [pnpm](https://pnpm.io/) (v8 o posterior)

### Descarga

Descargue la última versión desde la página de [Releases](https://github.com/mh3nj/evoury/releases).

### Construir desde el Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/mh3nj/evoury.git
cd evoury

# Instalar dependencias
pnpm install

# Iniciar servidor de desarrollo
pnpm tauri dev

# Construir para producción
pnpm tauri build
```

---

## Desarrollo

### Comandos Disponibles

```bash
# Desarrollo
pnpm dev              # Iniciar servidor Vite
pnpm tauri dev        # Iniciar Tauri en modo desarrollo

# Construcción
pnpm build            # Construir frontend
pnpm tauri build      # Construir aplicación Tauri

# Pruebas
pnpm test             # Ejecutar pruebas frontend
cargo test            # Ejecutar pruebas Rust

# Lint
pnpm lint             # Ejecutar ESLint
cargo clippy          # Ejecutar Clippy

# Formato
pnpm format           # Formatear código frontend
cargo fmt             # Formatear código Rust
```

---

## Stack Tecnológico

### Backend

- **Rust** - Lenguaje de programación de sistemas
- **Tauri** - Framework de aplicaciones de escritorio
- **SQLite** - Base de datos local
- **Crossbeam** - Primitivas de programación concurrente

### Frontend

- **React** - Biblioteca de UI
- **TypeScript** - JavaScript con tipos
- **Tailwind CSS** - Framework CSS utilitario
- **Zustand** - Gestión de estado
- **Vite** - Herramienta de construcción y servidor de desarrollo

---

## Hoja de Ruta

Consulte [ROADMAP.md](ROADMAP.md) para la hoja de ruta detallada.

---

## Contribuir

¡Las contribuciones son bienvenidas! Por favor, lea [CONTRIBUTING.md](CONTRIBUTING.md) primero.

---

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo [LICENSE](LICENSE) para más detalles.

---

## Soporte

- **Problemas**: [GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **Discusiones**: [GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  Hecho con ❤️ por <a href="https://github.com/mh3nj">Mohsen Jafari</a>
</p>
