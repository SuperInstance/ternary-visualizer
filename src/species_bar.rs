use std::fmt::Write;

/// Horizontal bar chart showing species distribution.
pub struct SpeciesBar {
    /// Species name and population count.
    species: Vec<(String, usize)>,
    width: usize,
}

impl SpeciesBar {
    /// Create from a list of (name, count) pairs.
    pub fn new(species: Vec<(impl Into<String>, usize)>) -> Self {
        let species = species.into_iter().map(|(n, c)| (n.into(), c)).collect();
        Self { species, width: 40 }
    }

    /// Set the bar width.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w.max(10);
        self
    }

    /// Render the bar chart.
    pub fn render(&self) -> String {
        let total: usize = self.species.iter().map(|(_, c)| *c).sum();
        if total == 0 {
            return "(no species data)\n".to_string();
        }

        let max_name_len = self.species.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
        let max_count = self.species.iter().map(|(_, c)| *c).max().unwrap_or(1);
        let mut out = String::new();

        out.push_str(&format!("{:>width$} ┤\n", "", width = max_name_len));

        for (name, count) in &self.species {
            let pct = *count as f64 / total as f64;
            let bar_len = (*count as f64 / max_count as f64 * self.width as f64).round() as usize;
            let bar: String = "█".repeat(bar_len);

            out.push_str(&format!(
                "{:>name_w$} │{} ({}, {:.0}%)\n",
                name,
                bar,
                count,
                pct * 100.0,
                name_w = max_name_len,
            ));
        }

        out.push_str(&format!("{:>width$} ┤\n", "", width = max_name_len));
        out.push_str(&format!(
            "{:>name_w$} └ Total: {}\n",
            "",
            total,
            name_w = max_name_len,
        ));

        out
    }

    /// Render a compact single-line version.
    pub fn render_compact(&self) -> String {
        let total: usize = self.species.iter().map(|(_, c)| *c).sum();
        if total == 0 {
            return "∅".to_string();
        }

        let mut out = String::new();
        for (i, (name, count)) in self.species.iter().enumerate() {
            if i > 0 { write!(out, " | ").unwrap(); }
            let pct = *count as f64 / total as f64 * 100.0;
            write!(out, "{}: {} ({:.0}%)", name, count, pct).unwrap();
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_bar_chart() {
        let bar = SpeciesBar::new(vec![
            ("Species A", 50),
            ("Species B", 30),
            ("Species C", 20),
        ]);
        let rendered = bar.render();
        assert!(rendered.contains("Species A"));
        assert!(rendered.contains("Species B"));
        assert!(rendered.contains("Species C"));
        assert!(rendered.contains("Total: 100"));
    }

    #[test]
    fn test_empty_species() {
        let bar = SpeciesBar::new(Vec::<(&str, usize)>::new());
        assert_eq!(bar.render(), "(no species data)\n");
    }

    #[test]
    fn test_compact_render() {
        let bar = SpeciesBar::new(vec![("A", 75), ("B", 25)]);
        let compact = bar.render_compact();
        assert!(compact.contains("A: 75 (75%)"));
        assert!(compact.contains("B: 25 (25%)"));
    }
}
