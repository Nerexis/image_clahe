# Image CLAHE Processor

A Rust program that applies Contrast Limited Adaptive Histogram Equalization (CLAHE) to images to enhance contrast. Supports both standard and High Dynamic Range (HDR) images. For HDR images make sure to use proper HDR extension in file names.

## Build

Ensure you have Rust and OpenCV installed on your system.

```bash
# Build the project in release mode
cargo build --release
```

## Run

```bash
cargo run --release -- <input_image_path> <output_image_path>
```

## Acknowledgment
This project uses OpenCV, which is licensed under the [Apache License 2.0](https://opensource.org/licenses/Apache-2.0).

## Donations
Say thank you by donating: https://revolut.me/damiann48/5usd

