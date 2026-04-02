# Tower Defense

A tower defense game built with [Bevy](https://bevyengine.org/) in Rust, featuring multiple game modes and multiplayer server support.

## Features

- Grid-based tower defense gameplay on a 10x10 map
- Four tower types with distinct mechanics
- Two map modes: fixed path and dynamic pathfinding
- Multiplayer server with up to 64 concurrent clients
- Godot engine integration via GDExtension

## Tower Types

| Tower | Cost | Range | Mechanic |
|-------|------|-------|----------|
| Basic | 50g | 25 | Single-target damage |
| Bomb | 100g | 20 | Area-of-effect explosions |
| Follower | 75g | 50 | Homing missiles |
| Slow | 10g | 50 | Applies speed debuff |

Turrets support multiple targeting strategies: Weakest, Strongest, Slowest, Fastest, Closest, or Furthest.

## Game Modes

- **FixedPathMap** (default) — Creeps follow a pre-set path through a maze-like map
- **FreeMap** — Open map where creeps use A* pathfinding and recalculate routes around placed towers
- **Server** — Headless multiplayer server mode

## Prerequisites

**Rust toolchain** (stable) plus system libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev libudev-dev libwayland-dev pkg-config protobuf-compiler
```

## Building & Running

```bash
# Build all crates
cargo build

# Run (FixedPathMap mode)
cargo run --bin tower_defense

# Run FreeMap mode
cargo run --bin tower_defense -- freemap

# Run multiplayer server (listens on 0.0.0.0:5000)
cargo run --bin tower_defense -- server
```

## Controls

| Input | Action |
|-------|--------|
| Left click | Place Basic turret |
| Right click | Place Follower turret |
| Middle click | Place Bomb turret |

## Project Structure

```
tower_defense/              # Binary entry point
tower_defense_plugin/       # Core game logic (ECS components & systems)
tower_defense_gui/          # Bevy rendering and input handling
tower_defense_server/       # Network server (bevy_renet / UDP)
protos/                     # Protocol Buffer definitions
godot_lib/                  # Godot GDExtension integration
```

## Testing & Linting

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## License

MIT — Copyright 2025 Cezuriku
