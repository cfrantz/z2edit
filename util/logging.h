#ifndef Z2HD_UTIL_LOGGING_H
#define Z2HD_UTIL_LOGGING_H
#include <cstdio>
#include <string>
#include <unistd.h>
#include "absl/strings/str_cat.h"


#define LOG(LEVEL, ...) \
    logging::Log(logging::LogLevel::LL_##LEVEL, __VA_ARGS__)
#define LOGF(LEVEL, ...) \
    logging::LogF(logging::LogLevel::LL_##LEVEL, __VA_ARGS__)
#define HEX(...) logging::Hex(__VA_ARGS__)

namespace logging {

extern const int RESET;
extern const int BOLD;
extern const int BLACK;
extern const int RED;
extern const int GREEN;
extern const int YELLOW;
extern const int BLUE;
extern const int MAGENTA;
extern const int CYAN;
extern const int WHITE;

enum LogLevel {
    LL_FATAL = 0,
    LL_ERROR = 1,
    LL_WARN = 2,
    LL_WARNING = 2,
    LL_INFO = 3,
    LL_VERBOSE = 4,
};

extern void logging_init();
extern int logging_init_done;
extern LogLevel loglevel;
extern FILE* logfp;
extern int logfp_isatty;

extern void SetLogColor(int color);
extern std::string Hex(uint8_t x, bool lz=true, bool zx=true);
extern std::string Hex(uint16_t x, bool lz=true, bool zx=true);
extern std::string Hex(uint32_t x, bool lz=true, bool zx=true);
extern std::string Hex(uint64_t x, bool lz=true, bool zx=true);

inline std::string Hex(int8_t x, bool lz=true, bool zx=true) {
    return Hex(uint8_t(x), lz, zx);
}
inline std::string Hex(int16_t x, bool lz=true, bool zx=true) {
    return Hex(uint16_t(x), lz, zx);
}
inline std::string Hex(int32_t x, bool lz=true, bool zx=true) {
    return Hex(uint32_t(x), lz, zx);
}
inline std::string Hex(int64_t x, bool lz=true, bool zx=true) {
    return Hex(uint64_t(x), lz, zx);
}
inline std::string Hex(void* x, bool lz=true, bool zx=true) {
    return Hex(intptr_t(x), lz, zx);
}

template<typename ...Args>
void Log(LogLevel level, Args ...args) {
    if (!logging_init_done)
        logging_init();

    if (level > loglevel)
        return;

    int color = WHITE;
    const char* prefix = "[?] ";
    switch(level) {
        case LL_FATAL:
            prefix = "[F] ";
            color = RED;
            break;
        case LL_ERROR:
            prefix = "[E] ";
            color = RED;
            break;
        case LL_WARN:
            prefix = "[W] ";
            color = YELLOW;
            break;
        case LL_INFO:
            prefix = "[I] ";
            color = BLUE;
            break;
        case LL_VERBOSE:
            prefix = "[V] ";
            color = GREEN;
            break;
        default:
            ; // Do nothing
    }

    if (logfp_isatty) {
        SetLogColor(color);
        fputs(prefix, logfp);
        fputs(absl::StrCat(args...).c_str(), logfp);
        SetLogColor(RESET);
        fputs("\n", logfp);
    } else {
        fputs(prefix, logfp);
        fputs(absl::StrCat(args...).c_str(), logfp);
        fputs("\n", logfp);
    }
    if (level == LL_FATAL) {
        abort();
    }
}

void LogF(LogLevel level, const char *fmt, ...);

}  // namespace

#endif // Z2HD_UTIL_LOGGING_H
