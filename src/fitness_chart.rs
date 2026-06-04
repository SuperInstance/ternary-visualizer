/// ASCII line chart of fitness values over generations.
///
/// Renders a sparkline-style chart using Unicode block characters.
pub struct FitnessChart {
    values: Vec<f64>,
    width: usize,
    height: usize,
}

impl FitnessChart {
    /// Create a new chart with the given fitness values per generation.
    pub fn new(values: Vec<f64>) -> Self {
        Self { values, width: 60, height: 10 }
    }

    /// Set the display width.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w.max(10);
        self
    }

    /// Set the display height.
    pub fn height(mut self, h: usize) -> Self {
        self.height = h.max(3);
        self
    }

    /// Render the chart as a string.
    pub fn render(&self) -> String {
        if self.values.is_empty() {
            return "| (no data)\n+---------\n".to_string();
        }

        let min = self.values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = if (max - min).abs() < f64::EPSILON { 1.0 } else { max - min };

        let mut out = String::new();

        // Y-axis labels and plot
        for row in (0..self.height).rev() {
            let y_val = min + range * row as f64 / (self.height - 1).max(1) as f64;
            out.push_str(&format!("{:5.2} │", y_val));

            for (i, val) in self.values.iter().enumerate() {
                if i >= self.width { break; }
                let normalized = (val - min) / range;
                let row_pos = normalized * (self.height - 1) as f64;
                let diff = row_pos - row as f64;
                if diff >= 0.0 && diff < 1.0 {
                    out.push('█');
                } else if diff >= -0.5 && diff < 0.0 {
                    out.push('▄');
                } else if diff >= 1.0 && diff < 1.5 {
                    out.push('▀');
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }

        // X-axis
        out.push_str("      └");
        for _ in 0..self.values.len().min(self.width) {
            out.push('─');
        }
        out.push('\n');

        // X labels
        out.push_str(&format!(
            "       0{}{}\n",
            " ".repeat(self.values.len().min(self.width).saturating_sub(4)),
            self.values.len() - 1
        ));

        out
    }

    /// Render a compact sparkline (single line).
    pub fn render_sparkline(&self) -> String {
        if self.values.is_empty() {
            return "▁".to_string();
        }
        let min = self.values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = if (max - min).abs() < f64::EPSILON { 1.0 } else { max - min };

        let spark_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        self.values.iter().map(|v| {
            let norm = ((v - min) / range).clamp(0.0, 1.0);
            let idx = (norm * (spark_chars.len() - 1) as f64).round() as usize;
            spark_chars[idx]
        }).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_chart() {
        let chart = FitnessChart::new(vec![]);
        let rendered = chart.render();
        assert!(rendered.contains("no data"));
    }

    #[test]
    fn test_basic_chart() {
        let chart = FitnessChart::new(vec![0.1, 0.3, 0.5, 0.7, 0.9]);
        let rendered = chart.render();
        assert!(rendered.contains("│"));
        assert!(rendered.contains("└"));
        assert!(rendered.contains("█"));
    }

    #[test]
    fn test_sparkline() {
        let chart = FitnessChart::new(vec![0.0, 0.5, 1.0]);
        let spark = chart.render_sparkline();
        assert!(spark.starts_with('▁'));
        assert!(spark.ends_with('█'));
        assert_eq!(spark.chars().count(), 3);
    }

    #[test]
    fn test_sparkline_empty() {
        let chart = FitnessChart::new(vec![]);
        assert_eq!(chart.render_sparkline(), "▁");
    }

    #[test]
    fn test_constant_values() {
        let chart = FitnessChart::new(vec![0.5, 0.5, 0.5]);
        let spark = chart.render_sparkline();
        // All equal values map to the max sparkline character
        assert!(spark.chars().all(|c| c == '█' || c == '▁'));
        assert_eq!(spark.chars().count(), 3);
    }
}
