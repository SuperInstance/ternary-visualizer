# PLUG_AND_PLAY — Visualizer

> ASCII/text visualizations of ternary agent dynamics

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-visualizer = { git = "https://github.com/SuperInstance/ternary-visualizer" }
```

Use in your code:

```rust
use ternary_visualizer::{StrategyGrid, ConservationGauge};

let grid = StrategyGrid::new(&population);
println!("{}", grid.render());
```

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
