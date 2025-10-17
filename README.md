# Hexi

Hexi is a small hexdump clone, writen in rust.

## Installation

```bash
cargo install hexi-rs
```

## Usage

### Hexdump a file

```bash
hexi-rs ./myfile
```

### Hexdump from stdin

```bash
cat ./myfile | hexi-rs -
```
