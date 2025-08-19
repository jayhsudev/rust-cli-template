# Rust Cli Template

## Step

Install cargo-generate:

```bash
cargo install cargo-generate
```

Generate into a subfolder:

```bash
cargo generate --git https://github.com/jayhsudev/rust-cli-template.git
```

Generate in the current folder:

```bash
cargo generate --init --git https://github.com/jayhsudev/rust-cli-template.git
```

## Features

- Layered configuration with [config-rs](https://github.com/mehcode/config-rs)
- Command line argument parser and shell completions with [clap](https://github.com/clap-rs/clap)
- XDG support with [directories](https://github.com/dirs-dev/directories-rs)
- Logging and tracing with [tracing](https://github.com/tokio-rs/tracing)
- Return code error propagation with [miette](https://github.com/zkat/miette)
- Async runtime with graceful shutdown support with [tokio](https://github.com/tokio-rs/tokio)
