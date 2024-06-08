# ncmc &middot; [![Test](https://github.com/magic-akari/ncmc/actions/workflows/test.yml/badge.svg)](https://github.com/magic-akari/ncmc/actions/workflows/test.yml) [![Crates.io](https://img.shields.io/crates/v/ncmc.svg?label=ncmc)](https://crates.io/crates/ncmc)

## Install

### Option 1: Download from GitHub Release

For users who prefer a pre-built binary, you can download the latest release from [![GitHub Release](https://img.shields.io/badge/build-Release-brightgreen?style=flat&logo=github&label=GitHub)](https://github.com/magic-akari/ncmc/releases/latest)

### Option 2: Install with Cargo

If you have Rust installed, you can install ncmc with cargo:

```bash
cargo install ncmc
```

Additionally, [cargo binstall](https://github.com/cargo-bins/cargo-binstall) is supported.
It fetch the pre-built binary from GitHub Release and fallback to build from source if not available.

```bash
cargo binstall ncmc
```

If you don’t have cargo, install it with
https://rustup.rs

## Usage

```bash
# convert
ncmc path/to/your/file.ncm

# dump mode
ncmc --dump path/to/your/file.ncm
```

---

Thanks: [anonymous5l / ncmdump](https://github.com/anonymous5l/ncmdump)
