int PushStyleVar(const proto::StyleVar& v) {
    int pushes = 0;
    if (v.alpha_size() != 0) {
        if (v.alpha_size() == 1) {
            float val = v.alpha(0);
            ImGui::PushStyleVar(ImGuiStyleVar_Alpha, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "alpha: expecting exactly 1 item, but got " << v.alpha_size();
        }
    }
    if (v.disabled_alpha_size() != 0) {
        if (v.disabled_alpha_size() == 1) {
            float val = v.disabled_alpha(0);
            ImGui::PushStyleVar(ImGuiStyleVar_DisabledAlpha, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "disabled_alpha: expecting exactly 1 item, but got " << v.disabled_alpha_size();
        }
    }
    if (v.window_padding_size() != 0) {
        if (v.window_padding_size() == 2) {
            ImVec2 val(v.window_padding(0), v.window_padding(1));
            ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "window_padding: expecting exactly 2 items, but got " << v.window_padding_size();
        }
    }
    if (v.window_rounding_size() != 0) {
        if (v.window_rounding_size() == 1) {
            float val = v.window_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "window_rounding: expecting exactly 1 item, but got " << v.window_rounding_size();
        }
    }
    if (v.window_border_size_size() != 0) {
        if (v.window_border_size_size() == 1) {
            float val = v.window_border_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_WindowBorderSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "window_border_size: expecting exactly 1 item, but got " << v.window_border_size_size();
        }
    }
    if (v.window_min_size_size() != 0) {
        if (v.window_min_size_size() == 2) {
            ImVec2 val(v.window_min_size(0), v.window_min_size(1));
            ImGui::PushStyleVar(ImGuiStyleVar_WindowMinSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "window_min_size: expecting exactly 2 items, but got " << v.window_min_size_size();
        }
    }
    if (v.window_title_align_size() != 0) {
        if (v.window_title_align_size() == 2) {
            ImVec2 val(v.window_title_align(0), v.window_title_align(1));
            ImGui::PushStyleVar(ImGuiStyleVar_WindowTitleAlign, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "window_title_align: expecting exactly 2 items, but got " << v.window_title_align_size();
        }
    }
    if (v.child_rounding_size() != 0) {
        if (v.child_rounding_size() == 1) {
            float val = v.child_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_ChildRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "child_rounding: expecting exactly 1 item, but got " << v.child_rounding_size();
        }
    }
    if (v.child_border_size_size() != 0) {
        if (v.child_border_size_size() == 1) {
            float val = v.child_border_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_ChildBorderSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "child_border_size: expecting exactly 1 item, but got " << v.child_border_size_size();
        }
    }
    if (v.popup_rounding_size() != 0) {
        if (v.popup_rounding_size() == 1) {
            float val = v.popup_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_PopupRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "popup_rounding: expecting exactly 1 item, but got " << v.popup_rounding_size();
        }
    }
    if (v.popup_border_size_size() != 0) {
        if (v.popup_border_size_size() == 1) {
            float val = v.popup_border_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_PopupBorderSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "popup_border_size: expecting exactly 1 item, but got " << v.popup_border_size_size();
        }
    }
    if (v.frame_padding_size() != 0) {
        if (v.frame_padding_size() == 2) {
            ImVec2 val(v.frame_padding(0), v.frame_padding(1));
            ImGui::PushStyleVar(ImGuiStyleVar_FramePadding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "frame_padding: expecting exactly 2 items, but got " << v.frame_padding_size();
        }
    }
    if (v.frame_rounding_size() != 0) {
        if (v.frame_rounding_size() == 1) {
            float val = v.frame_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_FrameRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "frame_rounding: expecting exactly 1 item, but got " << v.frame_rounding_size();
        }
    }
    if (v.frame_border_size_size() != 0) {
        if (v.frame_border_size_size() == 1) {
            float val = v.frame_border_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_FrameBorderSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "frame_border_size: expecting exactly 1 item, but got " << v.frame_border_size_size();
        }
    }
    if (v.item_spacing_size() != 0) {
        if (v.item_spacing_size() == 2) {
            ImVec2 val(v.item_spacing(0), v.item_spacing(1));
            ImGui::PushStyleVar(ImGuiStyleVar_ItemSpacing, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "item_spacing: expecting exactly 2 items, but got " << v.item_spacing_size();
        }
    }
    if (v.item_inner_spacing_size() != 0) {
        if (v.item_inner_spacing_size() == 2) {
            ImVec2 val(v.item_inner_spacing(0), v.item_inner_spacing(1));
            ImGui::PushStyleVar(ImGuiStyleVar_ItemInnerSpacing, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "item_inner_spacing: expecting exactly 2 items, but got " << v.item_inner_spacing_size();
        }
    }
    if (v.indent_spacing_size() != 0) {
        if (v.indent_spacing_size() == 1) {
            float val = v.indent_spacing(0);
            ImGui::PushStyleVar(ImGuiStyleVar_IndentSpacing, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "indent_spacing: expecting exactly 1 item, but got " << v.indent_spacing_size();
        }
    }
    if (v.cell_padding_size() != 0) {
        if (v.cell_padding_size() == 2) {
            ImVec2 val(v.cell_padding(0), v.cell_padding(1));
            ImGui::PushStyleVar(ImGuiStyleVar_CellPadding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "cell_padding: expecting exactly 2 items, but got " << v.cell_padding_size();
        }
    }
    if (v.scrollbar_size_size() != 0) {
        if (v.scrollbar_size_size() == 1) {
            float val = v.scrollbar_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_ScrollbarSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "scrollbar_size: expecting exactly 1 item, but got " << v.scrollbar_size_size();
        }
    }
    if (v.scrollbar_rounding_size() != 0) {
        if (v.scrollbar_rounding_size() == 1) {
            float val = v.scrollbar_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_ScrollbarRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "scrollbar_rounding: expecting exactly 1 item, but got " << v.scrollbar_rounding_size();
        }
    }
    if (v.grab_min_size_size() != 0) {
        if (v.grab_min_size_size() == 1) {
            float val = v.grab_min_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_GrabMinSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "grab_min_size: expecting exactly 1 item, but got " << v.grab_min_size_size();
        }
    }
    if (v.grab_rounding_size() != 0) {
        if (v.grab_rounding_size() == 1) {
            float val = v.grab_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_GrabRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "grab_rounding: expecting exactly 1 item, but got " << v.grab_rounding_size();
        }
    }
    if (v.tab_rounding_size() != 0) {
        if (v.tab_rounding_size() == 1) {
            float val = v.tab_rounding(0);
            ImGui::PushStyleVar(ImGuiStyleVar_TabRounding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "tab_rounding: expecting exactly 1 item, but got " << v.tab_rounding_size();
        }
    }
    if (v.button_text_align_size() != 0) {
        if (v.button_text_align_size() == 2) {
            ImVec2 val(v.button_text_align(0), v.button_text_align(1));
            ImGui::PushStyleVar(ImGuiStyleVar_ButtonTextAlign, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "button_text_align: expecting exactly 2 items, but got " << v.button_text_align_size();
        }
    }
    if (v.selectable_text_align_size() != 0) {
        if (v.selectable_text_align_size() == 2) {
            ImVec2 val(v.selectable_text_align(0), v.selectable_text_align(1));
            ImGui::PushStyleVar(ImGuiStyleVar_SelectableTextAlign, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "selectable_text_align: expecting exactly 2 items, but got " << v.selectable_text_align_size();
        }
    }
    if (v.separator_text_border_size_size() != 0) {
        if (v.separator_text_border_size_size() == 1) {
            float val = v.separator_text_border_size(0);
            ImGui::PushStyleVar(ImGuiStyleVar_SeparatorTextBorderSize, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "separator_text_border_size: expecting exactly 1 item, but got " << v.separator_text_border_size_size();
        }
    }
    if (v.separator_text_align_size() != 0) {
        if (v.separator_text_align_size() == 2) {
            ImVec2 val(v.separator_text_align(0), v.separator_text_align(1));
            ImGui::PushStyleVar(ImGuiStyleVar_SeparatorTextAlign, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "separator_text_align: expecting exactly 2 items, but got " << v.separator_text_align_size();
        }
    }
    if (v.separator_text_padding_size() != 0) {
        if (v.separator_text_padding_size() == 2) {
            ImVec2 val(v.separator_text_padding(0), v.separator_text_padding(1));
            ImGui::PushStyleVar(ImGuiStyleVar_SeparatorTextPadding, val);
            pushes +=1;
        } else {
            LOG(ERROR) << "separator_text_padding: expecting exactly 2 items, but got " << v.separator_text_padding_size();
        }
    }
    return pushes;
}
