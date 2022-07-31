# pxls-board-getter-rust

## Motivation

An exercise in learning how to fetch remote data, parse JSON (partially) into a struct, and write a PNG using Rust.

## Usage

```
./pxls-board-getter-rust --path FILE
```
Path will default to `canvas.png`.

## Building

```
cargo build --release
```
The output executable is in `target/release/`, titled `pxls-board-getter-rust`. Extension depends on the operating system.

## Roadmap

- Add support for heatmap, virginmap, and placemap.
