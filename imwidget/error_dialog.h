#ifndef Z2UTIL_IMWIDGET_ERROR_DIALOG_H
#define Z2UTIL_IMWIDGET_ERROR_DIALOG_H
#include <string>
#include <functional>

#include "util/strutil.h"

class ErrorDialog {
  public:
    static ErrorDialog* New(const std::string& title, int buttons,
                            const std::string& message);

    template<typename ...Args>
    static ErrorDialog* New(const std::string& title,
                            const std::string& message, Args ...args) {
        return New(title, DISMISS, StrCat(message, args...));
    }

    template<typename ...Args>
    static ErrorDialog* New(const std::string& title, int buttons,
                            const std::string& message, Args ...args) {
        return New(title, buttons, StrCat(message, args...));
    }

    ErrorDialog(const std::string& title, int buttons,
                const std::string& message)
        : visible_(true), popup_(false), buttons_(buttons), result_(0),
          title_(title), message_(message) {}

    void Draw();
    inline bool* visible() { return &visible_; }
    inline int result() const { return result_; }
    inline void set_result_cb(std::function<void(int)> result_cb) {
        result_cb_ = result_cb;
    }
    const static int DISMISS = 1;
    const static int OK = 2;
    const static int CANCEL = 4;
  private:
    const static int MAX_BUTTONS = 3;
    bool visible_;
    bool popup_;
    int buttons_;
    int result_;
    std::string title_;
    std::string message_;
    std::function<void(int)> result_cb_;
};

#endif // Z2UTIL_IMWIDGET_ERROR_DIALOG_H
