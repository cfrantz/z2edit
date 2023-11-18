#include "util/fontawesome.h"

#include "imgui.h"
#include "util/file.h"

absl::Status LoadFontAwesome() {
    ImGuiIO& io = ImGui::GetIO();
    io.Fonts->AddFontDefault();
    static const ImWchar icons_ranges[] = {ICON_MIN_FA, ICON_MAX_FA, 0};
    ImFontConfig icons_config;
    icons_config.MergeMode = true;
    icons_config.PixelSnapH = true;
    auto* result = io.Fonts->AddFontFromFileTTF(
        "third_party/fonts/fontawesome/fa-solid-900.otf", 16.0f, &icons_config,
        icons_ranges);
    if (result == nullptr) {
        return absl::UnknownError("Failed to load font");
    }

    return absl::OkStatus();
}
