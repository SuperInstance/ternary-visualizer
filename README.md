# ternary-visualizer

**ASCII art for ternary signals. Make the invisible visible, right in your terminal.**

Not everything needs a GUI. Sometimes you just need to *see* the data — quick, dirty, in the terminal where you're already working. This crate renders ternary populations, fitness landscapes, species distributions, conservation gauges, and cascade timelines as ASCII art. Block characters, bar charts, heat maps, and sparklines. No dependencies. No rendering. Just text.

Each visualization uses the three ternary characters: `░` (avoid/empty/negative), `▒` (unknown/neutral/zero), `█` (choose/full/positive). Together they create patterns you can read at a glance — population density, species boundaries, fitness gradients, cascade propagation.

## What's Inside

- **`PopulationHeatmap`** — render a ternary population grid as ASCII block characters
- **`FitnessChart`** — bar chart of fitness values across a population
- **`SpeciesBar`** — horizontal bars showing species distribution (avoid/unknown/choose)
- **`ConservationGauge`** — a single-line gauge showing how well conservation laws hold
- **`CascadeTimeline`** — show cascade events over time as a vertical timeline
- **`StrategyGrid`** — render a strategy space as a 2D grid with ASCII cells
- **`SummaryCard`** — a compact dashboard card combining multiple visualizations

## Quick Example

```rust
use ternary_visualizer::*;

// Population heatmap: see the spatial distribution
let mut heatmap = PopulationHeatmap::new(20, 10);
heatmap.set(5, 3, TernaryState::Choose);
heatmap.set(6, 3, TernaryState::Choose);
heatmap.set(5, 4, TernaryState::Avoid);
println!("{}", heatmap.render());
// ░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░
// ░░░░░░░█░░░░░░░░░░░░
// ░░░░░░░░░░░░░░░░░░░░
// ...

// Species bar: population composition at a glance
let bar = SpeciesBar::new(avoid_count, unknown_count, choose_count);
println!("{}", bar.render());
// ░░░░▒▒▒▒▒▒▒████████

// Conservation gauge: is the system healthy?
let gauge = ConservationGauge::new(0.87); // 87% conservation
println!("{}", gauge.render());
// ╢████████████████░░░╣ 87%
```

## The Deeper Truth

**ASCII visualization is the fastest path from data to understanding.** No build step, no rendering engine, no window system. You print text, you see patterns. The three ternary characters (░▒█) create a visual language that's immediately readable: dark regions are dense with activity, light regions are empty, the boundary between them is where things get interesting.

This crate is also the most "Plato room" of all — it's a self-contained visualization toolkit that any other crate can use for debugging, logging, or documentation. Every other crate in the fleet could `println!("{}", visualize(&data))` and get instant feedback. No setup required.

**Use cases:**
- **Debugging** — visualize agent populations during development
- **Logging** — render state snapshots in log files for post-hoc analysis
- **Documentation** — include ASCII visualizations in READMEs and papers
- **Terminal dashboards** — real-time monitoring of running systems
- **Education** — show students what's happening inside the simulation

## See Also

- **ternary-color** — for when ASCII isn't enough (actual color output)
- **ternary-life** — the grid CA that produces the most dramatic ASCII patterns
- **ternary-fire** — fire grids look incredible as ASCII heatmaps
- **ternary-vu** — audio metering (another visualization challenge)
- **ternary-gauge** — numerical statistics that feed the gauges

## Install

```bash
cargo add ternary-visualizer
```

## License

MIT
