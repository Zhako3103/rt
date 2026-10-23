# rt

A **multithreaded CPU ray tracer built from scratch in pure Rust**. It has no external dependencies: parallelism uses `std::thread::scope`, and the image is written to a plain PPM file.

## Features

- **Primitives:** sphere, plane, **cube with arbitrary rotation** (ray/box intersection with the *slab method* in the cube's local space) and cylinder.
- **Lighting:** Blinn-Phong shading (ambient, diffuse, specular).
- **Shadows:** shadow rays toward each light.
- **Reflections:** recursive reflection rays with a bounded depth.
- **Anti-aliasing:** multiple samples per pixel.
- **Gamma correction** before writing the output.
- **Multithreading:** the image is split into rows/tiles rendered concurrently with scoped threads.
- **Camera:** configurable position, direction and field of view.

## Build

```bash
cargo build --release
```

## Usage

```bash
# Render the default scene
cargo run --release -- --width 800 --height 600 --output scene.ppm

# Higher quality: more samples per pixel, more reflection bounces
cargo run --release -- --width 1280 --height 720 --samples 16 --depth 5 --output scene_hq.ppm
```

| Flag | Description |
| --- | --- |
| `--width`, `--height` | Output resolution in pixels |
| `--samples` | Anti-aliasing samples per pixel |
| `--depth` | Maximum reflection depth |
| `--threads` | Number of worker threads (defaults to available cores) |
| `--output` | Output file name (`.ppm`) |

Convert to PNG with ImageMagick if needed:

```bash
convert scene.ppm scene.png
```

## Rendering pipeline

1. For every pixel, generate camera rays (several per pixel for anti-aliasing).
2. Find the nearest intersection among all objects.
3. Compute Blinn-Phong lighting, tracing a shadow ray to every light.
4. Spawn a reflection ray for reflective materials and blend the result.
5. Apply gamma correction and write the pixel.

## What this project demonstrates

Applied linear algebra, geometric intersection tests, shared-nothing parallelism with `std::thread::scope`, and numeric care (self-intersection bias, gamma-correct output).
