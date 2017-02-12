#ifndef Z2UTIL_IMAPP_UTIL_H
#define Z2UTIL_IMAPP_UTIL_H
#include <string>
#include <functional>
#include <memory>
#include "imwidget/imwidget.h"

// FIXME(cfrantz): Refactor ImApp into a generic base class and a specialized
// subclass with application logic, then eliminate this in favor of
// something like:
//
// ImApp::Get()->AddDrawCallback(...);
void AddDrawCallback(ImWindowBase* window);

void Help(const std::string& topickey);
void HelpButton(const std::string& topickey);
#endif // Z2UTIL_IMAPP_UTIL_H
