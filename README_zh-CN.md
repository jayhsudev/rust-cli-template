# 🦀 Rust CLI 模板

[![License](https://img.shields.io/github/license/jayhsudev/rust-cli-template)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)
[![cargo-generate](https://img.shields.io/badge/cargo--generate-template-orange.svg)](https://github.com/cargo-generate/cargo-generate)

> 🚀 现代化、功能丰富的 Rust CLI 应用程序模板，内置最佳实践。

[English](README.md) | 中文

## ✨ 特性

此模板提供构建专业 Rust CLI 应用程序所需的一切：

- 🔧 **分层配置** - 使用 [config-rs](https://github.com/mehcode/config-rs) 进行灵活的配置管理
- 🎯 **命令行解析** - 使用 [clap](https://github.com/clap-rs/clap) 进行强大的参数解析和 shell 补全
- 📁 **XDG 目录支持** - 使用 [directories](https://github.com/dirs-dev/directories-rs) 进行跨平台目录处理
- 📊 **结构化日志** - 使用 [tracing](https://github.com/tokio-rs/tracing) 进行全面的日志记录和追踪
- 🎨 **优雅错误处理** - 使用 [miette](https://github.com/zkat/miette) 提供用户友好的错误消息
- ⚡ **异步运行时** - 使用 [tokio](https://github.com/tokio-rs/tokio) 提供高性能异步支持和优雅关闭

## 📦 快速开始

### 前置要求

首先，安装 `cargo-generate`：

```bash
cargo install cargo-generate
```

### 使用方法

**生成到新的子文件夹：**
```bash
cargo generate --git https://github.com/jayhsudev/rust-cli-template.git
```

**在当前目录生成：**
```bash
cargo generate --init --git https://github.com/jayhsudev/rust-cli-template.git
```

## 🏗️ 项目结构

```
your-cli-app/
├── src/
│   ├── main.rs          # 应用程序入口点
│   ├── cli.rs           # 命令行接口定义
│   ├── config.rs        # 配置管理
│   ├── log.rs           # 日志设置
│   └── commands/        # 命令实现
│       ├── mod.rs
│       ├── command1.rs
│       ├── command2.rs
│       └── completion.rs
├── config/
│   └── config.toml      # 默认配置
├── build.rs             # 补全功能的构建脚本
└── Cargo.toml          # 项目依赖
```

## 🚀 你将获得什么

- **现代 Rust**：使用 Rust 2024 版本和最新最佳实践
- **Shell 补全**：自动生成 bash、zsh、fish 和 PowerShell 的补全
- **配置文件**：基于 TOML 的配置，支持环境变量
- **结构化命令**：清晰的命令结构，支持子命令
- **优雅关闭**：正确的信号处理和资源清理
- **跨平台**：支持 Windows、macOS 和 Linux

## 🛠️ 开发

生成项目后：

```bash
# 构建项目
cargo build

# 使用调试日志运行
RUST_LOG=debug cargo run

# 生成 shell 补全
cargo run -- completion bash > completions.bash

# 运行测试
cargo test

# 检查代码质量
cargo clippy
cargo fmt
```

## 📝 配置

模板包含分层配置系统：

1. **默认值** 在 `config/config.toml` 中
2. **环境变量**（以你的应用名称为前缀）
3. **命令行参数**（最高优先级）

配置结构示例：
```toml
[app]
name = "my-cli-app"
version = "0.1.0"

[logging]
level = "info"
file = "app.log"
```

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📄 许可证

本项目基于 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 使用令人惊叹的 Rust 生态系统构建，充满 ❤️
- 受现代 CLI 最佳实践启发
- 感谢所有 crate 维护者的出色工作
