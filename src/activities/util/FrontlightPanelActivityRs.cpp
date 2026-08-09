#include "FrontlightPanelActivityRs.h"

#include <I18n.h>

#include "I18nKeys.h"

FrontlightPanelActivityRs::FrontlightPanelActivityRs(GfxRenderer& renderer, MappedInputManager& mappedInput)
    : ActivityRs("FrontlightPanelRs", renderer, mappedInput, create_frontlight_panel_activity) {}

const char* FrontlightPanelActivityRs::getTitle() const { return tr(STR_FRONTLIGHT); }
