# Rust containerd shim v2 for runh container

This shim is a fork of [io.containerd.runc.v2-rs](https://github.com/containerd/rust-extensions/blob/main/crates/runc-shim) and adapted for [runh](https://github.com/hermit-os/runh).
By default [containerd](https://github.com/containerd/containerd) relies on runtime shim to launch containers.

## Usage

To build binary, run:
```shell
cargo build --release
```
