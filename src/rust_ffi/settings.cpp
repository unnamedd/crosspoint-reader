// Persisted settings Rust screens read or write. Deliberately narrow: a general
// settings writer would hand every screen the ability to write flash.

#include <HalDisplay.h>

#include "CrossPointSettings.h"
#include "internal.h"

extern "C" {

uint8_t cpp_dev_frontlight_panel_rust() { return SETTINGS.frontlightPanelRust; }

void cpp_dev_set_frontlight_panel_rust(const uint8_t enabled) {
  const uint8_t value = enabled ? 1 : 0;
  if (SETTINGS.frontlightPanelRust == value) return;  // no needless flash write
  SETTINGS.frontlightPanelRust = value;
  SETTINGS.saveToFile();
}

// The setting, not the panel's current polarity. ActivityManager re-resolves
// the display every render from `screenInverted && appliesNightMode()`, so a
// direct flip here is undone on the next frame and the state read back would
// contradict the setting. This mirrors FrontlightPanelActivity::toggleInversion.
uint8_t cpp_display_is_inverted() { return SETTINGS.screenInverted != 0 ? 1 : 0; }

uint8_t cpp_display_toggle_inverted() {
  const bool inverted = SETTINGS.screenInverted == 0;
  SETTINGS.screenInverted = inverted ? 1 : 0;
  SETTINGS.saveToFile();
  return inverted ? 1 : 0;
}
}
