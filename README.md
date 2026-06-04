# ternary-visualizer

ASCII/text visualizations of ternary agent dynamics — for terminals, logs, and docs.

Pure Rust, no unsafe code, no external dependencies.

## Visualizations

### PopulationHeatmap

Render population state as an ASCII heatmap using block characters:

```
+---------+
|Population|
+---------+
|░░▒▒▒█░░▒|
|░▒▒█░░▒░░|
|▒░░░▒█▒░░|
+---------+
```

- `░` = Avoid
- `▒` = Unknown  
- `█` = Choose

### FitnessChart

ASCII line chart of fitness over generations with Y-axis labels:

```
 0.90 │      █
 0.80 │    █
 0.70 │  █
 0.60 │ █
 0.50 │█
 0.40 │
      └─────
       0    4
```

Or as a compact sparkline: `▁▃▄▅▇█`

### SpeciesBar

Horizontal bar chart showing species distribution:

```
       ┤
Species A │████████████████████████████████████████ (50, 50%)
Species B │██████████████████████ (30, 30%)
Species C │██████████████ (20, 20%)
       ┤
       └ Total: 100
```

### ConservationGauge

Visual gauge showing conservation deviation from ideal:

```
Avoid: 0.670 (target: 0.333) ⚠
[░░░░░░░░░░░░░░░░░░░░░░░░░█│░░░░░░░░░░░░░░░·]
 0        25        50        75       100%
```

- `│` = target position
- `█` = actual position
- `◆` = on target (both overlap)

### CascadeTimeline

Timeline showing cascade events with markers:

```
Cascade Timeline (100 generations)
0                                                         100
│─────────────────────────────────────────────────────────────│
│··········▼···································●··············│
│
 ▼ Gen   10 │ ▼ Avoid           │ mag=0.80 │ First cascade
 ● Gen   50 │ ● Equil           │ mag=0.10 │ Equilibrium
```

Event types: `▼` Avoid cascade, `▲` Choose cascade, `✕` Extinction, `★` Emergence, `●` Equilibrium

### StrategyGrid

Render a strategy landscape as a 2D grid with symbols:

```
   0123456789...
 0 ░░▒▒░░▒▓█░
 1 ░▒▓░░▒▒▓░░
 2 ▒░░▓▓░░▒░░

  · Low fitness
  ░ Below avg
  ▒ Average
  ▓ Above avg
  █ Peak fitness
```

### SummaryCard

Compact one-line summary:

```
Gen 50 | Fitness 0.73 | Species: 5 | Avoid 67% | Unknown 10% | Choose 23% | Std 0.001 ✓
```

Status symbols: `✓` Healthy, `~` Warning, `⚠` Critical, `⟳` Running

## Usage

```rust
use ternary_visualizer::*;

// Population heatmap
let heatmap = PopulationHeatmap::from_counts(30, 20, 10, 10);
println!("{}", heatmap.render_with_border(Some("Population")));

// Fitness chart
let chart = FitnessChart::new(vec![0.1, 0.3, 0.5, 0.7, 0.9]);
println!("{}", chart.render());
println!("Sparkline: {}", chart.render_sparkline());

// Species bar chart
let bar = SpeciesBar::new(vec![("Alpha", 50), ("Beta", 30), ("Gamma", 20)]);
println!("{}", bar.render());

// Conservation gauge
let gauge = ConservationGauge::new("Avoid", 0.333, 0.333);
println!("{}", gauge.render());

// Cascade timeline
let events = vec![
    CascadeEvent { generation: 10, label: "Cascade".into(), magnitude: 0.8, event_type: CascadeEventType::AvoidCascade },
];
let timeline = CascadeTimeline::new(events, 100);
println!("{}", timeline.render());

// Strategy grid
let grid = StrategyGrid::from_fitness(vec![vec![0.1, 0.5, 0.9], vec![0.3, 0.7, 0.2]]);
println!("{}", grid.render());

// Summary card
let card = SummaryCard::new(50, 0.73)
    .species(5)
    .population(67.0, 10.0, 23.0)
    .std_deviation(0.001)
    .auto_status();
println!("{}", card.render());
```

## License

MIT
