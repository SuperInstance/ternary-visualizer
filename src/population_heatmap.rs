use crate::TernaryState;

/// Renders a population grid as an ASCII heatmap.
///
/// Each cell maps to a ternary state rendered as a block character:
/// - Avoid → ░
/// - Unknown → ▒
/// - Choose → █
pub struct PopulationHeatmap {
    grid: Vec<Vec<TernaryState>>,
    width: usize,
    height: usize,
}

impl PopulationHeatmap {
    /// Create a new heatmap from a 2D grid of ternary states.
    pub fn new(grid: Vec<Vec<TernaryState>>) -> Self {
        let height = grid.len();
        let width = grid.first().map(|r| r.len()).unwrap_or(0);
        Self { grid, width, height }
    }

    /// Create a flat heatmap from a slice of states with the given row width.
    pub fn from_flat(states: &[TernaryState], width: usize) -> Self {
        let grid: Vec<Vec<TernaryState>> = states.chunks(width).map(|c| c.to_vec()).collect();
        Self::new(grid)
    }

    /// Generate a random-ish population from counts.
    pub fn from_counts(avoid: usize, unknown: usize, choose: usize, width: usize) -> Self {
        let mut states = Vec::new();
        for _ in 0..avoid { states.push(TernaryState::Avoid); }
        for _ in 0..unknown { states.push(TernaryState::Unknown); }
        for _ in 0..choose { states.push(TernaryState::Choose); }
        Self::from_flat(&states, width)
    }

    /// Render the heatmap as a string.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for row in &self.grid {
            for state in row {
                out.push(state.symbol());
            }
            out.push('\n');
        }
        out
    }

    /// Render with a border and optional title.
    pub fn render_with_border(&self, title: Option<&str>) -> String {
        let mut out = String::new();
        let inner_w = self.width;

        // Top border
        out.push('+');
        for _ in 0..inner_w { out.push('-'); }
        out.push_str("+\n");

        // Title
        if let Some(t) = title {
            let padded = format!(" {} ", t);
            let left = (inner_w.saturating_sub(padded.len())) / 2;
            out.push('|');
            for _ in 0..left { out.push(' '); }
            out.push_str(&padded);
            for _ in 0..(inner_w.saturating_sub(left + padded.len())) { out.push(' '); }
            out.push_str("|\n");
            out.push('+');
            for _ in 0..inner_w { out.push('-'); }
            out.push_str("+\n");
        }

        // Grid rows
        for row in &self.grid {
            out.push('|');
            for state in row {
                out.push(state.symbol());
            }
            out.push_str("|\n");
        }

        // Bottom border
        out.push('+');
        for _ in 0..inner_w { out.push('-'); }
        out.push_str("+\n");

        out
    }

    /// Get the state counts.
    pub fn counts(&self) -> (usize, usize, usize) {
        let (mut a, mut u, mut c) = (0, 0, 0);
        for row in &self.grid {
            for s in row {
                match s {
                    TernaryState::Avoid => a += 1,
                    TernaryState::Unknown => u += 1,
                    TernaryState::Choose => c += 1,
                }
            }
        }
        (a, u, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_render() {
        let grid = vec![
            vec![TernaryState::Avoid, TernaryState::Unknown, TernaryState::Choose],
            vec![TernaryState::Choose, TernaryState::Avoid, TernaryState::Unknown],
        ];
        let hm = PopulationHeatmap::new(grid);
        let rendered = hm.render();
        assert!(rendered.contains("░▒█"));
        assert!(rendered.contains("█░▒"));
    }

    #[test]
    fn test_render_with_border() {
        let grid = vec![
            vec![TernaryState::Avoid; 5],
        ];
        let hm = PopulationHeatmap::new(grid);
        let rendered = hm.render_with_border(Some("Population"));
        assert!(rendered.starts_with('+'));
        assert!(rendered.contains("Population"));
        assert!(rendered.contains("░░░░░"));
    }

    #[test]
    fn test_counts() {
        let hm = PopulationHeatmap::from_counts(3, 2, 1, 3);
        assert_eq!(hm.counts(), (3, 2, 1));
    }

    #[test]
    fn test_from_flat() {
        let states = vec![TernaryState::Avoid; 9];
        let hm = PopulationHeatmap::from_flat(&states, 3);
        assert_eq!(hm.height, 3);
        assert_eq!(hm.width, 3);
        let rendered = hm.render();
        assert_eq!(rendered.trim(), "░░░\n░░░\n░░░");
    }
}
