ImGuiSliderFlags GetSliderFlags(const proto::SliderFlags& flags) {
    ImGuiSliderFlags f = 0;
    if (flags.always_clamp()) f |= ImGuiSliderFlags_AlwaysClamp;
    if (flags.logarithmic()) f |= ImGuiSliderFlags_Logarithmic;
    if (flags.no_round_to_format()) f |= ImGuiSliderFlags_NoRoundToFormat;
    if (flags.no_input()) f |= ImGuiSliderFlags_NoInput;
    return f;
}
