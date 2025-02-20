# Rust containerd shim v2 for runh container

This shim is a fork of [io.containerd.runc.v2-rs](https://github.com/containerd/rust-extensions/blob/main/crates/runc-shim) and adapted for [runh](https://github.com/hermit-os/runh).
By default [containerd](https://github.com/containerd/containerd) relies on runtime shim to launch containers.

## Usage

To build binary, run:
```shell
cargo build --release
```

Copy binary to the containerd shim dir, e.g. `/usr/bin/`

In order to use it from containerd, use:

```shell
sudo ctr run --runtime "io.containerd.runh.v2" --rm -t ghcr.io/hermit-os/rusty_demo:latest demo
```

# Funding

The development of this project was partially funded by the European Union's Horizon Europe research and innovation programme under grant agreement No. 101070118 - NEMO.
