#ifndef Z2UTIL_IMAPP_UTIL_H
#define Z2UTIL_IMAPP_UTIL_H
#include <functional>

// FIXME(cfrantz): Refactor ImApp into a generic base class and a specialized
// subclass with application logic, then eliminate this in favor of
// something like:
//
// ImApp::Get()->AddDrawCallback(...);
void AddDrawCallback(std::function<bool()> cb);

#endif // Z2UTIL_IMAPP_UTIL_H
