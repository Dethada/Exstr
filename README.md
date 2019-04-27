# Chars

Display printable characters in file, similar to strings but prints any printable character instead of finding strings.

## Building 🔨

```bash
cargo build --release
```

## Install

### Linux 🐧
```bash
make install
```

### Windows
Add `target/release/chars` to somewhere in path.

## Usage 📖

```bash
➜ chars --help
chars 0.1
David Z. <david@dzhy.dev>
Display printable characters in file

USAGE:
    chars --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    The target file
```
