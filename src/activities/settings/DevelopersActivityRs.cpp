#include "DevelopersActivityRs.h"

#include <I18n.h>

#include "I18nKeys.h"

DevelopersActivityRs::DevelopersActivityRs(GfxRenderer& renderer, MappedInputManager& mappedInput)
    : ActivityRs("DevelopersRs", renderer, mappedInput, create_developers_activity) {}

const char* DevelopersActivityRs::getTitle() const { return tr(STR_DEVELOPERS); }
