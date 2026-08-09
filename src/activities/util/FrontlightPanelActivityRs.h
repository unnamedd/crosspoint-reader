#pragma once

#include "activities/ActivityRs.h"

// Forward declaration for Rust FFI factory function
extern "C" {
void* create_frontlight_panel_activity();
}

/// Rust implementation of the frontlight quick panel.
///
/// Behaviourally equivalent to FrontlightPanelActivity; which one opens is
/// chosen by SETTINGS.frontlightPanelRust (Settings > System > Developers).
/// Both are built so the two can be compared on the same device.
class FrontlightPanelActivityRs final : public ActivityRs {
 public:
  explicit FrontlightPanelActivityRs(GfxRenderer& renderer, MappedInputManager& mappedInput);

  const char* getTitle() const override;
};
