#include "imwidget/error_dialog.h"
#include "imgui.h"
#include "imapp-util.h"


ErrorDialog* ErrorDialog::New(const std::string& title,
                              int buttons,
                              const std::string& message) {
    ErrorDialog* err = new ErrorDialog(title, buttons, message);
    AddDrawCallback([err]() {
        bool vis = err->visible_;
        if (vis) {
            err->Draw();
        } else {
            delete err;
        }
        return vis;
    });
    return err;
}

void ErrorDialog::Draw() {
    const char *buttons[] = {
        "Dismiss", "OK", "Cancel",
    };
    if (!visible_)
        return;

    if (!popup_) {
        ImGui::OpenPopup(title_.c_str());
        popup_ = true;
    }
    if (ImGui::BeginPopupModal(title_.c_str())) {
        ImGui::Text("%s", message_.c_str());

        ImGui::Text("\n");
        ImGui::Text("\n");
        for(int i=0; i<MAX_BUTTONS; i++) {
            ImGui::SameLine();
            if (buttons_ & (1UL << i)) {
                if (ImGui::Button(buttons[i])) {
                    result_ |= (1UL << i);
                }
            }
        }
        if (result_) {
            ImGui::CloseCurrentPopup();
            if (result_cb_) result_cb_(result_);
            visible_ = false;
        }
        ImGui::EndPopup();
    }
}
