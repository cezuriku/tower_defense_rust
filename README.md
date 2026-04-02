# Tower Defense (Rust)

A tower defense game built with [Bevy](https://bevyengine.org/) — an ECS-based game engine in Rust. Supports both single-player and multiplayer modes, with optional Godot Engine integration.

## Features

- **Grid-based maps** — 10×10 grid with two map types: fixed-path and free-placement
- **Four turret types** — Basic, Bomb (AoE), Follower (homing), and Slow (debuff)
- **Six targeting strategies** — Weakest, Strongest, Slowest, Fastest, Closest, Furthest
- **Dynamic pathfinding** — A\* pathfinding that updates when the map changes
- **Multiplayer server** — UDP networking via `bevy_renet` with Protocol Buffers serialization
- **Godot integration** — cdylib for embedding game logic into Godot projects

## Turret Reference

| Type     | Cost | Range | Reload | Notes               |
|----------|------|-------|--------|---------------------|
| Basic    | 50   | 25    | 1.0s   | Standard projectile |
| Bomb     | 100  | 20    | 1.0s   | Area-of-effect      |
| Follower | 75   | 50    | 1.0s   | Homing bullets      |
| Slow     | 10   | 50    | 3.0s   | Reduces creep speed |

Players start with **500 gold** and **10 lives**.

## Prerequisites

**System dependencies (Ubuntu/Debian):**
```bash
sudo apt-get install -y libasound2-dev libudev-dev libwayland-dev pkg-config protobuf-compiler
```

**Rust toolchain:** 1.93.1+ (stable)

## Building

```bash
cargo build          # debug
cargo build --release  # optimized
```

## Running

```bash
# Single-player: fixed path map (default)
cargo run

# Single-player: free placement map
cargo run -- freemap

# Multiplayer server (listens on UDP port 5000)
cargo run -- server
```

## Testing & Linting

```bash
cargo test --all-features
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## Project Structure

```
tower_defense/          # Main binary & game assets
tower_defense_plugin/   # Core game logic (ECS systems, components, pathfinding)
tower_defense_gui/      # Rendering and input handling
tower_defense_server/   # Multiplayer server plugin
godot_lib/              # Godot Engine integration (cdylib)
protos/                 # Protocol Buffer message definitions
```

## License

MIT — see [LICENSE.md](LICENSE.md).
