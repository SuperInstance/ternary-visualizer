use crate::TernaryState;

/// Strategy grid — renders a 2D strategy landscape with symbols.
///
/// Each cell can represent a strategy or its dominance.
pub struct StrategyGrid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    legend: Vec<(char, String)>,
}

impl StrategyGrid {
    /// Create from a 2D grid of characters.
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid.first().map(|r| r.len()).unwrap_or(0);
        Self { grid, width, height, legend: vec![] }
    }

    /// Create from ternary states (mapped to symbols).
    pub fn from_ternary(states: Vec<Vec<TernaryState>>) -> Self {
        let grid: Vec<Vec<char>> = states.into_iter()
            .map(|row| row.into_iter().map(|s| s.symbol()).collect())
            .collect();
        Self::new(grid)
    }

    /// Create a strategy landscape from fitness values.
    ///
    /// Maps continuous values to a gradient of characters.
    pub fn from_fitness(values: Vec<Vec<f64>>) -> Self {
        let gradient = ['·', '░', '▒', '▓', '█'];
        let grid: Vec<Vec<char>> = values.into_iter().map(|row| {
            row.into_iter().map(|v| {
                let idx = (v.clamp(0.0, 1.0) * (gradient.len() - 1) as f64).round() as usize;
                gradient[idx]
            }).collect()
        }).collect();
        let mut sg = Self::new(grid);
        sg.legend = vec![
            ('·', "Low fitness".into()),
            ('░', "Below avg".into()),
            ('▒', "Average".into()),
            ('▓', "Above avg".into()),
            ('█', "Peak fitness".into()),
        ];
        sg
    }

    /// Set the legend entries.
    pub fn legend(mut self, entries: Vec<(char, String)>) -> Self {
        self.legend = entries;
        self
    }

    /// Render the grid.
    pub fn render(&self) -> String {
        let mut out = String::new();

        // Column headers (tens digit)
        out.push_str("   ");
        for x in (0..self.width).step_by(10) {
            out.push_str(&format!("{:<10}", x / 10));
        }
        out.push('\n');

        out.push_str("   ");
        for x in 0..self.width {
            out.push_str(&format!("{}", x % 10));
        }
        out.push('\n');

        for (y, row) in self.grid.iter().enumerate() {
            out.push_str(&format!("{:2} ", y));
            for c in row {
                out.push(*c);
            }
            out.push('\n');
        }

        // Legend
        if !self.legend.is_empty() {
            out.push('\n');
            for (sym, desc) in &self.legend {
                out.push_str(&format!("  {} {}\n", sym, desc));
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_grid() {
        let grid = StrategyGrid::new(vec![
            vec!['A', 'B', 'C'],
            vec!['D', 'E', 'F'],
        ]);
        let rendered = grid.render();
        assert!(rendered.contains("ABC"));
        assert!(rendered.contains("DEF"));
    }

    #[test]
    fn test_from_fitness() {
        let grid = StrategyGrid::from_fitness(vec![
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
        ]);
        let rendered = grid.render();
        assert!(rendered.contains("·"));
        assert!(rendered.contains("█"));
        assert!(rendered.contains("Peak fitness"));
    }

    #[test]
    fn test_from_ternary() {
        let grid = StrategyGrid::from_ternary(vec![
            vec![TernaryState::Avoid, TernaryState::Choose],
        ]);
        let rendered = grid.render();
        assert!(rendered.contains("░█"));
    }
}
