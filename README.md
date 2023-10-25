# FFMPEG crate for WebAssembly/WASI

This crate bundles FFMPEG's libraries, precompiled for WebAssembly. No native installation required.

Compatible with Fastly Compute.

Includes VP9 and AV1 encoders.

These are *low-level* bindings, directly exposing the original C functions to Rust.

## Usage

```toml
[dependencies]
ffmpeg-wasi = "0"
```

```rust
use ffmpeg_wasi::*;
```
