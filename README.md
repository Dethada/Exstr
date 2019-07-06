# Exstr

Get strings from file, similar to binutils strings.

## Building 🔨

```bash
cargo build --bin exstr --release
```

## Install

### Linux 🐧
```bash
make install
```

### Windows
Add `target/release/exstr` to somewhere in path.

## Usage 📖

```bash
➜ exstr --help
Exstr 0.1.0
David Z. <david@dzhy.dev>
Get strings from file

USAGE:
    exstr --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    The target file
```
