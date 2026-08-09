#pragma once

#include "activities/ActivityRs.h"

// Forward declaration for Rust FFI factory function
extern "C" {
void* create_developers_activity();
}

/// Developer switches and diagnostics (Settings > System > Developers).
///
/// Not a user feature: it hosts the frontlight panel implementation switch and
/// live heap figures, which exist for development rather than for reading.
class DevelopersActivityRs final : public ActivityRs {
 public:
  explicit DevelopersActivityRs(GfxRenderer& renderer, MappedInputManager& mappedInput);

  const char* getTitle() const override;
};
