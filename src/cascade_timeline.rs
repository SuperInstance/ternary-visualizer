use std::fmt::Write;

/// A cascade event in the timeline.
#[derive(Debug, Clone)]
pub struct CascadeEvent {
    pub generation: usize,
    pub label: String,
    pub magnitude: f64, // 0.0 - 1.0
    pub event_type: CascadeEventType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CascadeEventType {
    /// Mass shift toward avoid.
    AvoidCascade,
    /// Mass shift toward choose.
    ChooseCascade,
    /// Species extinction.
    Extinction,
    /// Species emergence.
    Emergence,
    /// Equilibrium reached.
    Equilibrium,
}

impl CascadeEventType {
    fn symbol(self) -> char {
        match self {
            CascadeEventType::AvoidCascade => '▼',
            CascadeEventType::ChooseCascade => '▲',
            CascadeEventType::Extinction => '✕',
            CascadeEventType::Emergence => '★',
            CascadeEventType::Equilibrium => '●',
        }
    }

    fn label(self) -> &'static str {
        match self {
            CascadeEventType::AvoidCascade => "Avoid",
            CascadeEventType::ChooseCascade => "Choose",
            CascadeEventType::Extinction => "Extinct",
            CascadeEventType::Emergence => "Emerge",
            CascadeEventType::Equilibrium => "Equil",
        }
    }
}

/// Timeline visualization of cascade events.
pub struct CascadeTimeline {
    events: Vec<CascadeEvent>,
    total_generations: usize,
    width: usize,
}

impl CascadeTimeline {
    /// Create a timeline with events and total generation count.
    pub fn new(events: Vec<CascadeEvent>, total_generations: usize) -> Self {
        Self { events, total_generations, width: 60 }
    }

    /// Set the display width.
    pub fn width(mut self, w: usize) -> Self {
        self.width = w.max(20);
        self
    }

    /// Render the timeline.
    pub fn render(&self) -> String {
        let mut out = String::new();

        if self.total_generations == 0 {
            out.push_str("(empty timeline)\n");
            return out;
        }

        // Header
        writeln!(out, "Cascade Timeline ({} generations)", self.total_generations).unwrap();
        out.push_str(&format!("0{}", " ".repeat(self.width - 4)));
        writeln!(out, "{}", self.total_generations).unwrap();

        // Timeline axis
        out.push('│');
        for _ in 1..self.width - 1 { out.push('─'); }
        out.push('│');
        out.push('\n');

        // Sort events by generation
        let mut sorted = self.events.clone();
        sorted.sort_by_key(|e| e.generation);

        // Event markers on the timeline
        let mut marker_line = vec![' '; self.width];
        for event in &sorted {
            let pos = (event.generation as f64 / self.total_generations as f64 * (self.width - 1) as f64).round() as usize;
            let pos = pos.min(self.width - 1);
            marker_line[pos] = event.event_type.symbol();
        }
        out.push('│');
        for &c in &marker_line[1..self.width - 1] {
            out.push(if c == ' ' { '·' } else { c });
        }
        out.push('│');
        out.push('\n');

        // Legend
        out.push_str("│");
        out.push('\n');

        for event in &sorted {
            let sym = event.event_type.symbol();
            let kind = event.event_type.label();
            writeln!(out,
                " {} Gen {:>4} │ {} {:16} │ mag={:.2} │ {}",
                sym, event.generation, sym, kind, event.magnitude, event.label
            ).unwrap();
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_timeline() {
        let tl = CascadeTimeline::new(vec![], 0);
        assert_eq!(tl.render(), "(empty timeline)\n");
    }

    #[test]
    fn test_basic_timeline() {
        let events = vec![
            CascadeEvent {
                generation: 10,
                label: "First cascade".into(),
                magnitude: 0.8,
                event_type: CascadeEventType::AvoidCascade,
            },
            CascadeEvent {
                generation: 50,
                label: "Equilibrium".into(),
                magnitude: 0.1,
                event_type: CascadeEventType::Equilibrium,
            },
        ];
        let tl = CascadeTimeline::new(events, 100);
        let rendered = tl.render();
        assert!(rendered.contains("100 generations"));
        assert!(rendered.contains("▼"));
        assert!(rendered.contains("●"));
        assert!(rendered.contains("First cascade"));
        assert!(rendered.contains("Equilibrium"));
    }
}
