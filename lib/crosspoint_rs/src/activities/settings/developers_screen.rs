//! The Developers screen: diagnostics that are not user features.
//!
//! Reached from Settings > System > Developers. Shows live heap figures — the
//! numbers no build-time size report can show, because Rust allocates through
//! the firmware heap and so contributes almost nothing to static sections.

use backend::{device, tr};
use cpui::{vstack, List, ListRow, NavigationScreen, Screen, Section, Theme, ThemeMetric, View};

use crate::units::Units;

/// Everything this screen can be told.
#[derive(Clone, Copy)]
pub enum Msg {
    /// Any memory row: the scale applies to all of them, so they share it.
    CycleUnits,
}

#[derive(Default)]
pub struct DevelopersScreen {
    units: Units,
}

impl DevelopersScreen {
    pub fn new() -> Self {
        DevelopersScreen::default()
    }

    /// Live heap figures, read fresh each time this is built.
    ///
    /// Every row cycles the scale rather than only the one touched: the three
    /// numbers mean little apart, and comparing them across different units
    /// would be worse than useless.
    fn memory(&self) -> List<Msg> {
        let heap = device::heap();
        let row = |label: &str, bytes: i32| {
            ListRow::new(label)
                .value(self.units.format(bytes))
                .on_tap(Msg::CycleUnits)
        };

        List::new()
            .push(row(tr!(STR_FREE_HEAP), heap.free))
            .push(row(tr!(STR_LARGEST_BLOCK), heap.largest_block))
            .push(row(tr!(STR_MIN_FREE_HEAP), heap.min_free))
    }
}

impl Screen for DevelopersScreen {
    type Message = Msg;

    fn body(&self) -> impl View<Msg> {
        let spacing = Theme::metric(ThemeMetric::VerticalSpacing);

        NavigationScreen::new(vstack![spacing;
            Section::new(tr!(STR_SECTION_MEMORY), self.memory()),
        ])
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::CycleUnits => self.units = self.units.next(),
        }
    }
}

backend::register_screen!(DevelopersScreen, create_developers_activity);
