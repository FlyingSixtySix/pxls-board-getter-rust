# pxls-board-getter-rust

## Motivation

An exercise in learning how to fetch remote data, parse JSON (partially) into a struct, and write a PNG using Rust.

## Usage

```
USAGE:
    pxls-board-getter-rust [OPTIONS]

OPTIONS:
    -c, --use-canvas-code    
    -h, --host <HOST>        [default: https://pxls.space]
        --help               Print help information
    -p, --path <PATH>        Path of the board PNG to save [default: canvas.png]
    -V, --version            Print version information
```
Path will default to `canvas.png`.

## Building

```
cargo build --release
```
The output executable is in `target/release/`, titled `pxls-board-getter-rust`. Extension depends on the operating system.

## Roadmap

- Add support for heatmap, virginmap, and placemap.
