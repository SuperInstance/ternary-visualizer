use std::fmt::Write;

/// Compact one-line summary card.
///
/// Example: `Gen 50 | Fitness 0.73 | Species: 5 | Avoid 67% | Std 0.001 ✓`
pub struct SummaryCard {
    generation: usize,
    fitness: f64,
    species_count: usize,
    avoid_pct: f64,
    unknown_pct: f64,
    choose_pct: f64,
    std_deviation: f64,
    status: SummaryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummaryStatus {
    /// Within acceptable bounds.
    Healthy,
    /// Minor deviation.
    Warning,
    /// Significant deviation.
    Critical,
    /// Still running.
    Running,
}

impl SummaryStatus {
    fn symbol(self) -> &'static str {
        match self {
            SummaryStatus::Healthy => "✓",
            SummaryStatus::Warning => "~",
            SummaryStatus::Critical => "⚠",
            SummaryStatus::Running => "⟳",
        }
    }
}

impl SummaryCard {
    /// Create a new summary card.
    pub fn new(generation: usize, fitness: f64) -> Self {
        Self {
            generation,
            fitness,
            species_count: 0,
            avoid_pct: 0.0,
            unknown_pct: 0.0,
            choose_pct: 0.0,
            std_deviation: 0.0,
            status: SummaryStatus::Running,
        }
    }

    /// Set species count.
    pub fn species(mut self, count: usize) -> Self {
        self.species_count = count;
        self
    }

    /// Set population percentages (avoid, unknown, choose).
    pub fn population(mut self, avoid: f64, unknown: f64, choose: f64) -> Self {
        self.avoid_pct = avoid;
        self.unknown_pct = unknown;
        self.choose_pct = choose;
        self
    }

    /// Set the standard deviation.
    pub fn std_deviation(mut self, dev: f64) -> Self {
        self.std_deviation = dev;
        self
    }

    /// Set the status.
    pub fn status(mut self, status: SummaryStatus) -> Self {
        self.status = status;
        self
    }

    /// Auto-determine status based on std_deviation.
    pub fn auto_status(mut self) -> Self {
        if self.std_deviation < 0.01 {
            self.status = SummaryStatus::Healthy;
        } else if self.std_deviation < 0.05 {
            self.status = SummaryStatus::Warning;
        } else {
            self.status = SummaryStatus::Critical;
        }
        self
    }

    /// Render the one-line summary.
    pub fn render(&self) -> String {
        format!(
            "Gen {} | Fitness {:.2} | Species: {} | Avoid {:.0}% | Unknown {:.0}% | Choose {:.0}% | Std {:.3} {}",
            self.generation,
            self.fitness,
            self.species_count,
            self.avoid_pct,
            self.unknown_pct,
            self.choose_pct,
            self.std_deviation,
            self.status.symbol(),
        )
    }

    /// Render a minimal version.
    pub fn render_minimal(&self) -> String {
        format!(
            "G{} F{:.2} S{} A{:.0}% Std{:.3}{}",
            self.generation,
            self.fitness,
            self.species_count,
            self.avoid_pct,
            self.std_deviation,
            self.status.symbol(),
        )
    }

    /// Render a boxed version.
    pub fn render_boxed(&self) -> String {
        let inner = self.render();
        let w = inner.len();
        let mut out = String::new();
        writeln!(out, "┌{}┐", "─".repeat(w + 2)).unwrap();
        writeln!(out, "│ {} │", inner).unwrap();
        writeln!(out, "└{}┘", "─".repeat(w + 2)).unwrap();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_summary() {
        let card = SummaryCard::new(50, 0.73)
            .species(5)
            .population(67.0, 10.0, 23.0)
            .std_deviation(0.001)
            .auto_status();
        let rendered = card.render();
        assert!(rendered.contains("Gen 50"));
        assert!(rendered.contains("Fitness 0.73"));
        assert!(rendered.contains("Species: 5"));
        assert!(rendered.contains("Avoid 67%"));
        assert!(rendered.contains("✓"));
    }

    #[test]
    fn test_minimal_summary() {
        let card = SummaryCard::new(10, 0.5)
            .species(3)
            .population(33.0, 34.0, 33.0)
            .std_deviation(0.02)
            .auto_status();
        let minimal = card.render_minimal();
        assert!(minimal.contains("G10"));
        assert!(minimal.contains("F0.50"));
        assert!(minimal.contains("~")); // Warning status
    }

    #[test]
    fn test_boxed_summary() {
        let card = SummaryCard::new(1, 0.0)
            .species(1)
            .population(33.0, 33.0, 34.0)
            .std_deviation(0.1)
            .status(SummaryStatus::Critical);
        let boxed = card.render_boxed();
        assert!(boxed.starts_with('┌'));
        assert!(boxed.contains("⚠"));
    }
}
