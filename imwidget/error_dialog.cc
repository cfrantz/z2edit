#include "imwidget/error_dialog.h"
#include "imgui.h"
#include "imapp-util.h"


ErrorDialog* ErrorDialog::New(const std::string& title,
                              const std::string& message) {
    ErrorDialog* err = new ErrorDialog(title, message);
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
    if (!visible_)
        return;

    if (!popup_) {
        ImGui::OpenPopup(title_.c_str());
        popup_ = true;
    }
    if (ImGui::BeginPopupModal(title_.c_str())) {
        ImGui::Text("%s", message_.c_str());

        if (ImGui::Button("Dismiss")) {
            ImGui::CloseCurrentPopup();
            visible_ = false;
        }
        ImGui::EndPopup();
    }
}
