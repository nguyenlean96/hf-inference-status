<div align="center">

# 🤖 Hugging Face Inference Status Tracker

**Monitor and compare Hugging Face inference services in real-time**

[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-%23FF0000.svg?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
[![Leptos](https://img.shields.io/badge/leptos-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://leptos.dev/)

</div>

<p align="center">
  <em>A desktop application to track and compare Hugging Face inference services with real-time pricing, throughput, and latency metrics.</em>
</p>

<div align="center">

[Quick Start](#-quick-start) • [Features](#-features) • [Installation](#-installation) • [Architecture](#-architecture) • [Contributing](#-contributing)

</div>

---

## ✨ Features

| Feature | Description |
|--------|-------------|
| 📊 **Real-time Monitoring** | Track Hugging Face inference services status and performance metrics |
| 📈 **Performance Comparison** | Compare models across different providers based on pricing, throughput, and latency |
| ⭐ **Favorites System** | Save and organize your preferred models for quick access |
| 🔍 **Smart Filtering** | Filter and sort models based on your preferences |
| 💻 **Native Desktop Experience** | Cross-platform desktop application with offline capabilities |
| 🎨 **Modern UI** | Clean, responsive interface built with Tailwind CSS |

## 🚀 Quick Start

### Pre-built Binaries

Download the latest release for your platform:

- [macOS](https://github.com/nguyenlean96/hg-inference-status/releases/latest/download/hg-inference-status_x64.dmg)
- [Windows](https://github.com/nguyenlean96/hg-inference-status/releases/latest/download/hg-inference-status_x64.msi)
- [Linux](https://github.com/nguyenlean96/hg-inference-status/releases/latest/download/hg-inference-status_aarch64.AppImage)

### From Source

```bash
# Clone the repository
git clone https://github.com/nguyenlean96/hg-inference-status.git
cd hg-inference-status

# Install dependencies
npm install
cargo install trunk tauri-cli

# Run in development mode
cargo tauri dev
```

## 🔧 Installation

### Prerequisites

- Rust (latest stable)
- Node.js and npm
- Trunk (Rust web application bundler)
- Tauri CLI

### Building from Source

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Trunk and Tauri CLI**:
   ```bash
   cargo install trunk tauri-cli
   ```

3. **Clone and setup the project**:
   ```bash
   git clone https://github.com/nguyenlean96/hg-inference-status.git
   cd hg-inference-status
   npm install
   ```

4. **Run in development mode**:
   ```bash
   cargo tauri dev
   ```

5. **Build for production**:
   ```bash
   cargo tauri build
   ```

## 🏗️ Architecture

<div align="center">

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Communication  │    │    Backend      │
│                 │    │                  │    │                 │
│  • Leptos       │◄──►│  • Tauri Commands│◄──►│  • Rust         │
│  • Tailwind CSS │    │  • IPC           │    │  • Polars       │
│  • WASM         │    │                  │    │  • Reqwest      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         ▲                       ▲                       ▲
         │              ┌────────┴────────┐               │
         └──────────────│   Data Flow     │───────────────┘
                        │                 │
                        │ • Web Scraping  │
                        │ • HTML Parsing  │
                        │ • DataFrame Ops │
                        └─────────────────┘
```

</div>

The application follows a modern desktop application architecture:

- **Frontend**: Built with [Leptos](https://leptos.dev/), a reactive Rust web framework that compiles to WebAssembly
- **Backend**: Rust-based [Tauri](https://tauri.app/) application with asynchronous data processing
- **Data Pipeline**: Web scraping → HTML parsing → DataFrame processing → UI rendering
- **State Management**: Reactive state management using Leptos stores

For more detailed information about the architecture, see [ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## 🚧 Roadmap

Planned features and enhancements:

- [ ] **Enhanced favorite management** with categorization and notes
- [ ] **Real-time notifications** for model service changes
- [ ] **Advanced table manipulation** (grouping, advanced filtering)
- [ ] **Historical data tracking** and visualization
- [ ] **Export functionality** (CSV, JSON, Excel)
- [ ] **Performance improvements** for large datasets

## 🤝 Contributing

We welcome contributions from the community! Here's how you can help:

### Getting Started

1. Fork the repository
2. Create a branch for your feature (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow Rust coding conventions and best practices
- Write tests for new features when possible
- Update documentation as needed
- Keep pull requests focused on a single feature or bug fix
- Ensure your code passes all existing tests

### Project Structure

```
hg-inference-status/
├── src/                    # Frontend code (Leptos components)
├── src-tauri/             # Backend code (Tauri application)
│   ├── src/               # Rust backend modules
│   └── Cargo.toml         # Backend dependencies
├── docs/                  # Documentation
├── Cargo.toml             # Frontend dependencies
├── package.json           # Node.js dependencies
└── Trunk.toml             # Build configuration
```

## 🐛 Issues

If you encounter any issues or have feature requests, please [create an issue](https://github.com/nguyenlean96/hg-inference-status/issues) on GitHub. When reporting bugs, please include:

- Your operating system and version
- Steps to reproduce the issue
- Expected vs. actual behavior
- Any relevant error messages

## 📄 License

This project is licensed under the [ISC License](https://opensource.org/licenses/ISC) - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) for the amazing framework to build secure desktop applications
- [Leptos](https://leptos.dev/) for the reactive Rust web framework
- [Polars](https://pola.rs/) for high-performance DataFrame operations
- [Hugging Face](https://huggingface.co/) for providing the inference services data
- [Rust Community](https://www.rust-lang.org/) for the incredible ecosystem

## ⭐ Support

If you find this project useful, please consider giving it a star on GitHub! ⭐

<div align="center">

**Made with ❤️ for the Hugging Face community**

</div>