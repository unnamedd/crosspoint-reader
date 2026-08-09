//! The Developers screen: switches and diagnostics that are not user features.
//!
//! Reached from Settings > System > Developers. Shows live heap figures — the
//! numbers no build-time size report can show, because Rust allocates through
//! the firmware heap and so contributes almost nothing to static sections.

use alloc::format;
use alloc::string::{String, ToString};

use backend::device;
use cpui::{vstack, List, ListRow, NavigationScreen, Screen, Section, Theme, ThemeMetric, View};

use backend::tr;

/// Everything this screen can be told.
#[derive(Clone, Copy)]
pub enum Msg {
    /// Any memory row: the unit applies to all of them, so they share it.
    CycleUnits,
}

/// How byte figures are displayed. Tapping any memory row cycles this for all
/// of them, so the numbers stay comparable with each other.
#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Units {
    Bytes,
    #[default]
    Kilobytes,
}

impl Units {
    fn next(self) -> Self {
        match self {
            Units::Bytes => Units::Kilobytes,
            Units::Kilobytes => Units::Bytes,
        }
    }

    /// Formats a byte count, always with its unit so the two modes are never
    /// ambiguous. KB is rounded to nearest rather than truncated.
    fn format(self, bytes: i32) -> String {
        match self {
            Units::Bytes => format!("{} B", grouped(bytes)),
            Units::Kilobytes => format!("{} KB", grouped((bytes + 512) / 1024)),
        }
    }
}

/// Digit grouping. `core` has no locale formatting, and a six-digit heap figure
/// is unreadable ungrouped.
fn grouped(value: i32) -> String {
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    if value < 0 {
        return format!("-{out}");
    }
    out
}

#[derive(Default)]
pub struct DevelopersScreen {
    units: Units,
}

impl DevelopersScreen {
    pub fn new() -> Self {
        DevelopersScreen::default()
    }
}

impl Screen for DevelopersScreen {
    type Message = Msg;

    fn body(&self) -> impl View<Msg> {
        // Rebuilt per frame, so the heap figures are always current.
        let heap = device::heap();
        let spacing = Theme::metric(ThemeMetric::VerticalSpacing);

        let memory = List::new()
            .push(
                ListRow::new(tr!(STR_FREE_HEAP))
                    .value(self.units.format(heap.free))
                    .on_tap(Msg::CycleUnits),
            )
            .push(
                ListRow::new(tr!(STR_LARGEST_BLOCK))
                    .value(self.units.format(heap.largest_block))
                    .on_tap(Msg::CycleUnits),
            )
            .push(
                ListRow::new(tr!(STR_MIN_FREE_HEAP))
                    .value(self.units.format(heap.min_free))
                    .on_tap(Msg::CycleUnits),
            );

        NavigationScreen::new(vstack![spacing;
            Section::new(tr!(STR_SECTION_MEMORY), memory),
        ])
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::CycleUnits => self.units = self.units.next(),
        }
    }
}

backend::register_screen!(DevelopersScreen, create_developers_activity);
