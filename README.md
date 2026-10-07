# bake-watermark: Typst Package for Image Watermarking

## Overview
This project provides a Typst package implemented in Rust and compiled to WebAssembly (WASM) that watermarks images by baking the watermark directly into the image pixels. The package exports a function `bake-img` that takes an image path, watermark text, opacity, and width, and returns the watermarked image.

## Usage
In your Typst document, you can use the package as follows:

```typst
#import "@preview/bake-watermark:0.1.0": *

#bake-img("input.png", watermark-text: "CONFIDENTIAL", opacity: 128)

// Or with custom width:
#bake-img("input.png", watermark-text: "PROPERTY OF ACME", opacity: 100, width: 50%)
```

## Example
See the package in action:

| Input Image | Output Image (Watermarked) |
|-------------|----------------------------|
| ![Input](assets/input.png) | ![Output](assets/output.png) |

## Project Structure
- `src/lib.typ`: Typst interface for the package
- `src/bake_watermark.wasm`: Compiled WASM plugin
- `src/lib.rs`: Rust source code for the WASM plugin
- `fonts/font.ttf`: Embedded font used for rendering the watermark text (Liberation Sans)
- `src/input.png`: Sample input image
- `src/main.pdf`: Generated output PDF with watermarked image
- `src/main.typ`: Example Typst document demonstrating the package usage
- `assets/`: Directory containing example images for documentation

## Dependencies
The plugin relies on the following Rust crates:
- `wasm-minimal-protocol`: For interfacing with Typst's plugin system
- `rusttype`: For rendering TrueType font glyphs
- `image`: For loading, manipulating, and saving images

## Building
To compile the WASM plugin, run:
```sh
cargo build --release
```
The resulting WASM file will be located at:
`target/wasm32-unknown-unknown/release/bake_watermark.wasm`
This file is automatically copied to `src/bake_watermark.wasm` during the build process.

## How It Works
1. The plugin loads the input image from the provided bytes.
2. It loads the embedded font (Liberation Sans) and calculates the appropriate font size so that the watermark text spans approximately 60% of the image width.
3. The text is centered horizontally and vertically on the image.
4. For each glyph, the plugin blends white text at the specified opacity onto the image.
5. The modified image is encoded as PNG and returned as bytes.

## Notes
- The watermark text is rendered in white. You can modify the color in the source code if needed.
- The opacity value should be between 0 (fully transparent) and 255 (fully opaque).
- The plugin currently only supports PNG output, but the input can be any format supported by the `image` crate (PNG, JPEG, etc.).
