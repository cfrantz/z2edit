#ifndef Z2UTIL_IMWIDGET_ERROR_DIALOG_H
#define Z2UTIL_IMWIDGET_ERROR_DIALOG_H
#include <string>
#include "util/strutil.h"

class ErrorDialog {
  public:
    static ErrorDialog* New(const std::string& title,
                            const std::string& message);

    template<typename ...Args>                                                       
    static ErrorDialog* New(const std::string& title,
                            const std::string& message, Args ...args) {
        return New(title, StrCat(message, args...));
    }                                                                                

    ErrorDialog(const std::string& title, const std::string& message)
        : visible_(true), popup_(false), title_(title), message_(message) {}

    void Draw();
    inline bool* visible() { return &visible_; }
  private:
    bool visible_;
    bool popup_;
    std::string title_;
    std::string message_;
};

#endif // Z2UTIL_IMWIDGET_ERROR_DIALOG_H
