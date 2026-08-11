//! The frontlight quick panel, in Rust.
//!
//! A drop-down over whatever is on screen: brightness and warmth sliders that
//! drive the light live, plus sun and moon toggles. Behaviour mirrors the C++
//! `FrontlightPanelActivity` so the two can be compared directly — see the
//! implementation toggle under Settings > System > Developers.

use alloc::format;

use backend::{tr, Display, Frontlight, IconRole};
use xpui::{
    finish_screen, hstack, vstack, Alignment, Button, IconToggle, OverlayPanel, Screen, Scrim,
    Spacer, Stepper, Text, Theme, ThemeMetric, View,
};

#[derive(Clone, Copy)]
pub enum Msg {
    /// A new absolute value, from dragging or tapping a track.
    Brightness(i32),
    Warmth(i32),
    /// A relative nudge, from the -/+ glyphs or the side buttons.
    StepBrightness(i32),
    StepWarmth(i32),
    /// The state a glyph is moving to, as the toggle reports it.
    Light(bool),
    Inverted(bool),
    Dismiss,
}

pub struct FrontlightPanel {
    brightness: i32,
    warmth: i32,
    light_on: bool,
    inverted: bool,
    has_warmth: bool,
}

impl Default for FrontlightPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl FrontlightPanel {
    pub fn new() -> Self {
        FrontlightPanel {
            brightness: Frontlight::brightness(),
            warmth: Frontlight::warmth(),
            light_on: Frontlight::is_on(),
            inverted: Display::is_inverted(),
            has_warmth: Frontlight::has_warmth(),
        }
    }
}

impl FrontlightPanel {
    /// Switches the light, telling the firmware only on a real change.
    fn set_light(&mut self, on: bool) {
        if self.light_on == on {
            return;
        }

        self.light_on = on;
        Frontlight::set_on(on);
    }

    /// Brightness doubles as an "I want light" intent, so raising it turns the
    /// light on. Warmth deliberately does not — that mirrors the C++ panel.
    fn set_brightness(&mut self, value: i32) {
        let next = value.clamp(0, 100);
        if next == self.brightness {
            return;
        }

        self.brightness = next;
        Frontlight::set_brightness(next);
        self.set_light(true);
    }

    fn set_warmth(&mut self, value: i32) {
        let next = value.clamp(0, 100);
        if next == self.warmth {
            return;
        }

        self.warmth = next;
        Frontlight::set_warmth(next);
    }
}

impl FrontlightPanel {
    /// The brightness reading, and the two glyphs standing for light and
    /// inversion. Filled marks the active state: a solid sun means light on.
    fn header_row(&self) -> impl View<Msg> {
        let gap = Theme::metric(ThemeMetric::VerticalSpacing);

        hstack![gap;
            Text::new(format!("{}  {}%", tr!(STR_BRIGHTNESS), self.brightness)),
            Spacer::new(),
            IconToggle::new(IconRole::Moon, self.inverted).on_change(Msg::Inverted),
            IconToggle::new(IconRole::Sun, self.light_on).on_change(Msg::Light),
        ]
        .align(Alignment::Center)
    }

    fn brightness_control(&self) -> impl View<Msg> {
        Stepper::new(self.brightness)
            .on_change(Msg::Brightness)
            .on_step(Msg::StepBrightness)
    }

    /// Warmth carries its own label, since it appears only on hardware that
    /// has a warm channel and would otherwise be an unexplained second slider.
    fn warmth_control(&self) -> impl View<Msg> {
        let gap = Theme::metric(ThemeMetric::VerticalSpacing);

        vstack![gap / 2;
            Text::new(format!("{}  {}%", tr!(STR_WARMTH), self.warmth)),
            Stepper::new(self.warmth)
                .on_change(Msg::Warmth)
                .on_step(Msg::StepWarmth),
        ]
    }
}

impl Screen for FrontlightPanel {
    type Message = Msg;

    fn is_overlay(&self) -> bool {
        true // the screen underneath stays visible, and a tap on it dismisses
    }

    fn body(&self) -> impl View<Msg> {
        let gap = Theme::metric(ThemeMetric::VerticalSpacing);

        OverlayPanel::new(
            vstack![gap;
                self.header_row(),
                self.brightness_control(),
            ]
            .push_if(self.has_warmth, self.warmth_control()),
        )
        .scrim(Scrim::Dim)
        .on_scrim_tap(Msg::Dismiss)
    }

    /// Confirm is the light switch wherever focus happens to be, as on the C++
    /// panel. Everything else is left to the runtime: Up/Down move focus
    /// between the brightness and warmth rows, and Left/Right nudge whichever
    /// holds it.
    fn on_key(&self, key: Button) -> Option<Msg> {
        match key {
            Button::Confirm => Some(Msg::Light(!self.light_on)),
            _ => None,
        }
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Brightness(value) => self.set_brightness(value),
            Msg::Warmth(value) => self.set_warmth(value),
            Msg::StepBrightness(delta) => self.set_brightness(self.brightness + delta),
            Msg::StepWarmth(delta) => self.set_warmth(self.warmth + delta),
            Msg::Light(on) => self.set_light(on),
            // The firmware owns this flag, so take its word for the new state
            // rather than the one the glyph assumed.
            Msg::Inverted(_) => self.inverted = Display::toggle_inverted(),
            Msg::Dismiss => finish_screen(),
        }
    }

    fn on_exit(&mut self) {
        // One write on the way out, not per adjustment: flash has a finite
        // erase budget. The host skips it when nothing changed.
        Frontlight::save();
    }

    fn handle_home_gesture(&mut self) -> bool {
        finish_screen();
        true // consumed: the gesture dismisses the panel, not the screen below
    }
}

backend::register_screen!(FrontlightPanel, create_frontlight_panel_activity);
