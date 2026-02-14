# 🦀 Rust CLI Template

[![License](https://img.shields.io/github/license/jayhsudev/rust-cli-template)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)
[![cargo-generate](https://img.shields.io/badge/cargo--generate-template-orange.svg)](https://github.com/cargo-generate/cargo-generate)

> 🚀 A modern, feature-rich Rust CLI application template with best practices baked in.

English | [中文](README_zh-CN.md)

## ✨ Features

This template provides everything you need to build a professional Rust CLI application:

- 🔧 **Layered Configuration** - Flexible configuration management with [config-rs](https://github.com/mehcode/config-rs)
- 🎯 **Command Line Parsing** - Powerful argument parsing and shell completions with [clap](https://github.com/clap-rs/clap)
- 📁 **XDG Directory Support** - Cross-platform directory handling with [directories](https://github.com/dirs-dev/directories-rs)
- 📊 **Structured Logging** - Comprehensive logging and tracing with [tracing](https://github.com/tokio-rs/tracing)
- 🎨 **Beautiful Error Handling** - User-friendly error messages with [miette](https://github.com/zkat/miette)
- ⚡ **Async Runtime** - High-performance async support with graceful shutdown using [tokio](https://github.com/tokio-rs/tokio)

## 📦 Quick Start

### Prerequisites

First, install `cargo-generate`:

```bash
cargo install cargo-generate
```

### Usage

**Generate into a new subfolder:**
```bash
cargo generate --git https://github.com/jayhsudev/rust-cli-template.git
```

**Generate in the current directory:**
```bash
cargo generate --init --git https://github.com/jayhsudev/rust-cli-template.git
```

## 🏗️ Project Structure

```
your-cli-app/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command line interface definition
│   ├── config.rs        # Configuration management
│   ├── log.rs           # Logging setup
│   └── commands/        # Command implementations
│       ├── mod.rs
│       ├── command1.rs
│       ├── command2.rs
│       └── completion.rs
├── config/
│   └── config.toml      # Default configuration
├── build.rs             # Build script for completions
└── Cargo.toml          # Project dependencies
```

## 🚀 What You Get

- **Modern Rust**: Uses Rust 2024 edition with latest best practices
- **Shell Completions**: Auto-generated completions for bash, zsh, fish, and PowerShell
- **Configuration Files**: TOML-based configuration with environment variable support
- **Structured Commands**: Clean command structure with subcommands support
- **Graceful Shutdown**: Proper signal handling and resource cleanup
- **Cross-platform**: Works on Windows, macOS, and Linux

## 🛠️ Development

After generating your project:

```bash
# Build the project
cargo build

# Run with debug logging
RUST_LOG=debug cargo run

# Generate shell completions
cargo run -- completion bash > completions.bash

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

## 📝 Configuration

The template includes a layered configuration system:

1. **Default values** in `config/config.toml`
2. **Environment variables** (prefixed with your app name)
3. **Command line arguments** (highest priority)

Example configuration structure:
```toml
[app]
name = "my-cli-app"
version = "0.1.0"

[logging]
level = "info"
file = "app.log"
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using the amazing Rust ecosystem
- Inspired by modern CLI best practices
- Thanks to all the crate maintainers for their excellent work
