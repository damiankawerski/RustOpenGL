# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # compile
cargo run            # build and launch the OpenGL window
cargo check          # fast type-check without linking
```

There are no tests or linting targets configured.

## Architecture

This is a Rust OpenGL 3.3 renderer using `glutin` (context/surface), `winit` (window/events), `gl` (raw bindings), and `glam` (math).

### Module responsibilities

| Module | Role |
|---|---|
| `main.rs` | Bootstraps the glutin/winit window, GL context, and swap interval; hands off to `App` |
| `app.rs` | `ApplicationHandler` impl — routes winit events (keyboard, mouse, resize, redraw) to `GLWidget` |
| `glwidget.rs` | Scene owner: holds all geometry, shaders, camera, and key state; drives `paint()` each frame |
| `glsl.rs` | `GLSLProgram` wrapper — compile/link shaders, set uniforms, RAII `DeleteProgram` on drop |
| `geometry.rs` | `Geometry` — owns a VAO + VBOs; `set_vertices`/`set_attribute`/`set_indices` upload data; RAII cleanup on drop |
| `primitives.rs` | Procedural mesh builders (`new_sphere_geometry`, `new_box_geometry`, etc.); defines the `Attributes` enum |
| `camera.rs` | First-person camera: yaw/pitch rotation, view matrix |
| `frame.rs` | Rigid-body transform (pos + up + forward → `Mat4`) used to position scene objects |

### Vertex attribute layout (location indices)

Defined in `primitives::Attributes` and matched in `vs.glsl`:

```
location 0 — VertexPosition (Vec3)
location 1 — VertexColor    (Vec3)
location 2 — VertexNormal   (Vec3)
```

`set_attribute(index, &[Vec3])` in `geometry.rs` handles binding and `VertexAttribPointer` for any of these slots.

### Shaders

Loaded at runtime from `src/shaders/vs.glsl` and `src/shaders/fs.glsl`. The vertex shader receives MVP matrices (`MVMat`, `ViewMat`, `ProjectionMat`) as uniforms. `MVMat` is the per-object model-view matrix set via `render_body`.

### Coordinate conventions

- World Y is up for camera movement; the ground plane is oriented in XZ (via `Frame` with `up = Z, forward = Y`).
- Sphere/cylinder/cone are built along the Z axis (poles at ±Z).
- `Frame::matrix()` produces a column-major `Mat4` with columns `[right, up, -forward, pos]`.
