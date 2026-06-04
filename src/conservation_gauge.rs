use std::fmt::Write;

/// Visual gauge showing conservation deviation from ideal.
pub struct ConservationGauge {
    /// Actual proportion (0.0 - 1.0).
    actual: f64,
    /// Target/ideal proportion (0.0 - 1.0).
    target: f64,
    /// Gauge width in characters.
    width: usize,
    /// Label for the metric.
    label: String,
}

impl ConservationGauge {
    /// Create a new gauge with actual and target values.
    pub fn new(label: impl Into<String>, actual: f64, target: f64) -> Self {
        Self {
            actual: actual.clamp(0.0, 1.0),
            target: target.clamp(0.0, 1.0),
            width: 40,
            label: label.into(),
        }
    }

    /// Set the gauge width.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w.max(10);
        self
    }

    /// Compute the deviation from target.
    pub fn deviation(&self) -> f64 {
        (self.actual - self.target).abs()
    }

    /// Compute status string.
    fn status(&self) -> &'static str {
        let dev = self.deviation();
        if dev < 0.01 { "✓" }
        else if dev < 0.05 { "~" }
        else if dev < 0.1 { "!" }
        else { "⚠" }
    }

    /// Render the gauge.
    pub fn render(&self) -> String {
        let mut out = String::new();

        let target_pos = (self.target * self.width as f64).round() as usize;
        let actual_pos = (self.actual * self.width as f64).round() as usize;

        // Label line
        writeln!(out, "{}: {:.3} (target: {:.3}) {}",
            self.label, self.actual, self.target, self.status()).unwrap();

        // Gauge bar
        out.push('[');
        for i in 0..self.width {
            if i == target_pos && i == actual_pos {
                out.push('◆');
            } else if i == actual_pos {
                out.push('█');
            } else if i == target_pos {
                out.push('│');
            } else if i < actual_pos {
                out.push('░');
            } else {
                out.push('·');
            }
        }
        out.push(']');
        writeln!(out).unwrap();

        // Tick marks
        out.push(' ');
        for i in (0..=self.width).step_by(10) {
            let pct = i as f64 / self.width as f64 * 100.0;
            write!(out, "{:.0}", pct).unwrap();
            if i + 10 <= self.width {
                let padding = 10 - format!("{:.0}", pct).len();
                for _ in 0..padding { out.push(' '); }
            }
        }
        writeln!(out, "%").unwrap();

        out
    }

    /// Render a compact single-line version.
    pub fn render_compact(&self) -> String {
        let status = self.status();
        format!("{}: {:.3}/{:.3} {}", self.label, self.actual, self.target, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_target() {
        let gauge = ConservationGauge::new("Avoid", 0.333, 0.333);
        let rendered = gauge.render();
        assert!(rendered.contains("✓"));
        assert!(rendered.contains("0.333"));
    }

    #[test]
    fn test_off_target() {
        let gauge = ConservationGauge::new("Avoid", 0.9, 0.333);
        let rendered = gauge.render();
        assert!(rendered.contains("⚠"));
    }

    #[test]
    fn test_deviation() {
        let gauge = ConservationGauge::new("Test", 0.5, 0.333);
        assert!((gauge.deviation() - 0.167).abs() < 0.01);
    }

    #[test]
    fn test_compact() {
        let gauge = ConservationGauge::new("Avoid", 0.333, 0.333);
        let compact = gauge.render_compact();
        assert!(compact.contains("Avoid"));
        assert!(compact.contains("✓"));
    }
}
