//! FFMPEG low-level bindings for Rust, precompiled for WebAssembly/WASI.
//!
//! This crate bundles FFMPEG's `avcodec` and `avformat` libraries, precompiled for WebAssembly. No native installation required.
//!
//! Compatible with Fastly's Compute@Edge.
//!
//! These are *low-level* bindings, directly exposing the original C functions to Rust.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ffmpeg-wasi = "0"
//! ```

#![allow(
    non_camel_case_types,
    clashing_extern_declarations,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes
)]

pub mod avcodec;
pub mod avfilter;
pub mod avformat;
pub mod avutil;
pub mod swscale;

use std::alloc::{GlobalAlloc, Layout};

/// Allocator backed by ffmpeg memory allocation funnctions
///
/// Can be set as a global allocator with:
///
/// ```rust
/// #[global_allocator]
/// static ALLOCATOR: FFMpegAllocator = FFMpegAllocator;
/// ```
pub struct FFMpegAllocator;

unsafe impl GlobalAlloc for FFMpegAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        avutil::av_malloc(layout.size()) as *mut _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        avutil::av_free(ptr as *mut _)
    }
}

#[global_allocator]
static ALLOCATOR: FFMpegAllocator = FFMpegAllocator;

#[test]
pub fn test_ffmpeg() {
    unsafe {
        avcodec::avcodec_register_all();
    }
}
