//! The Developers screen: diagnostics that are not user features.
//!
//! Reached from Settings > System > Developers. Chooses which frontlight panel
//! opens, and shows live heap figures — the numbers no build-time size report
//! can show, because Rust allocates through the firmware heap and so
//! contributes almost nothing to static sections.

use backend::{device, tr, DevSettings};
use xpui::{
    vstack, List, ListRow, NavigationScreen, Screen, Section, Theme, ThemeMetric, Toggle, View,
};

use crate::units::Units;

/// Everything this screen can be told.
#[derive(Clone, Copy)]
pub enum Msg {
    /// The framework hands back the state the toggle is moving to.
    RustPanel(bool),
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
}

impl DevelopersScreen {
    fn frontlight_panel_toggle(&self) -> Toggle<Msg> {
        Toggle::new(
            tr!(STR_FRONTLIGHT_PANEL_RUST),
            DevSettings::frontlight_panel_rust(),
            tr!(STR_STATE_ON),
            tr!(STR_STATE_OFF),
        )
        .on_change(Msg::RustPanel)
    }

    fn memory_usage_view(&self) -> List<Msg> {
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
            Section::new(tr!(STR_SECTION_EXPERIMENTAL), self.frontlight_panel_toggle()),
            Section::new(tr!(STR_SECTION_MEMORY), self.memory_usage_view()),
        ])
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::RustPanel(enabled) => DevSettings::set_frontlight_panel_rust(enabled),
            Msg::CycleUnits => self.units = self.units.next(),
        }
    }
}

backend::register_screen!(DevelopersScreen, create_developers_activity);
