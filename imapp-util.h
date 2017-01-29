#ifndef Z2UTIL_IMAPP_UTIL_H
#define Z2UTIL_IMAPP_UTIL_H
#include <string>
#include <functional>

// FIXME(cfrantz): Refactor ImApp into a generic base class and a specialized
// subclass with application logic, then eliminate this in favor of
// something like:
//
// ImApp::Get()->AddDrawCallback(...);
void AddDrawCallback(std::function<bool()> cb);

void Help(const std::string& topickey);
void HelpButton(const std::string& topickey);
#endif // Z2UTIL_IMAPP_UTIL_H
