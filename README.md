# WC_AWF (Autonomous Web Assembly Framework)

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange)](https://crates.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![Target](https://img.shields.io/badge/target-wasm32%20|%20native-purple)](#)

> **Autonomous Cross-Platform Native Engine** written in Pure Rust for WebAssembly, Android NDK, iOS Metal, and Desktop targets.

---

## Overview

`wc_awf` is a low-overhead, high-performance rendering engine designed to bypass high-level GUI overheads. By rendering directly to hardware surfaces or WebGL/Canvas2D contexts, it provides hard sandbox execution and maximum frames-per-second performance.

## Key Features

* **Zero HTML DOM Overhead**: Renders directly via Canvas2D/WebGL or Native Hardware Surfaces.
* **True Cross-Platform Engine**:
  * **WebAssembly**: `wasm32-unknown-unknown`
  * **Android**: `ANativeWindow` (NDK)
  * **iOS**: `Metal` surface pipeline
  * **Desktop**: Native X11, Wayland, and Win32 backends
* **Hardened Security**:
  * Custom linear memory allocator.
  * Encrypted memory-bridge compatibility.
* **Minimal Dependencies**: Pure Rust vector rasterization and built-in input event loop.

---

## Architecture

+-------------------------------------------------------+
|                    Your Application                   |
+-------------------------------------------------------+
|
v
+-------------------------------------------------------+
|                     WC_AWF Engine                     |
|  +------------------+          +-------------------+  |
|  |  Vector Engine   |          |    Input Loop     |  |
|  +------------------+          +-------------------+  |
+-------------------------------------------------------+
|
+------------------+------------------+
|                                     |
v                                     v
+---------------+                     +---------------+
| Web Surface   |                     | Native Surface|
| (Canvas2D/    |                     | (Vulkan/Metal/|
|  WebGL)       |                     |  X11/Wayland) |
+---------------+                     +---------------+


---

## Quickstart Guide

### 1. Add Dependency

Add `wc_awf` to your `Cargo.toml`:

```toml
[dependencies]
wc_awf = "0.1.0"

2. Basic Example Usage (WASM Target)
Rust

use wc_awf::render::{CanvasDriver, VectorShape, Color};

fn main() {
    let driver = CanvasDriver::new("my-canvas-id");

    let shapes = vec![
        VectorShape::Rectangle {
            x: 10.0,
            y: 10.0,
            w: 200.0,
            h: 150.0,
            color: Color { r: 255, g: 0, b: 0, a: 1.0 },
        },
        VectorShape::Text {
            body: "WC_AWF Engine Running".to_string(),
            x: 20.0,
            y: 50.0,
            size: 16,
            color: Color { r: 255, g: 255, b: 255, a: 1.0 },
        },
    ];

    driver.render(&shapes);
}

3. Building for WASM

To compile your crate for WebAssembly:
Bash

cargo build --target wasm32-unknown-unknown --release

Or build with wasm-pack:
Bash

wasm-pack build --target web

License

Distributed under the MIT License. See LICENSE for more information.

support@worldclass-ai.com    ask me funny # wc_awf
# wc_awf
