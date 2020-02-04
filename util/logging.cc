#include <stdarg.h>
#include "util/logging.h"
#ifdef _WIN32
#include <windows.h>
#endif

#include <gflags/gflags.h>

DEFINE_int32(loglevel, 4, "Logging level");
DEFINE_string(logfile, "", "Log to file");

namespace logging {

const int RESET  = -1;
const int BOLD   = -2;
const int BLACK   = 0;
const int BLUE    = 1;
const int GREEN   = 2;
const int CYAN    = 3;
const int RED     = 4;
const int MAGENTA = 5;
const int YELLOW  = 6;
const int WHITE   = 7;

const char _RESET[] = "\033[0m";
const char _BOLD[]  = "\033[1m";
const char _BLACK[]   = "\033[30m";
const char _RED[]     = "\033[31m";
const char _GREEN[]   = "\033[32m";
const char _YELLOW[]  = "\033[33m";
const char _BLUE[]    = "\033[34m";
const char _MAGENTA[] = "\033[35m";
const char _CYAN[]    = "\033[36m";
const char _WHITE[]   = "\033[37m";

int logging_init_done;
LogLevel loglevel;
FILE* logfp;
int logfp_isatty;
#ifdef _WIN32
HANDLE hStdErr;
#endif

void logging_init() {
    if (logging_init_done)
        return;

    bool initerror = false;
    loglevel = LogLevel(FLAGS_loglevel);
    if (FLAGS_logfile.empty()) {
        logfp = stderr;
#ifdef _WIN32
        hStdErr = GetStdHandle(STD_ERROR_HANDLE);
#endif
    } else {
        logfp = fopen(FLAGS_logfile.c_str(), "w");
        if (logfp == nullptr) {
            initerror = true;
            logfp = stderr;
        }
    }
    logfp_isatty = isatty(fileno(logfp));
    if (initerror) {
        LOG(FATAL, "Could not open ", FLAGS_logfile, " for writing.");
    }
}

void SetLogColor(int color) {
#ifdef _WIN32
    static int bold;
    static int lastcolor = WHITE;
    if (color == RESET) {
        bold = 0;
        color = WHITE;
    } else if (color == BOLD) {
        bold = 8;
        color = lastcolor;
    }
    lastcolor = color;
    SetConsoleTextAttribute(hStdErr, bold+color);
#else
    const char *code;
    switch(color) {
        case RESET   : code = _RESET; break;
        case BOLD    : code = _BOLD; break;
        case BLACK   : code = _BLACK; break;
        case RED     : code = _RED; break;
        case GREEN   : code = _GREEN; break;
        case YELLOW  : code = _YELLOW; break;
        case BLUE    : code = _BLUE; break;
        case MAGENTA : code = _MAGENTA; break;
        case CYAN    : code = _CYAN; break;
        case WHITE   : code = _WHITE; break;
        default:
            code = _WHITE;
    }
    fputs(code, logfp);
#endif
}

const char hexletters[] = "0123456789ABCDEF";
inline char *HexHelper(char *buf, uint8_t x, bool lz) {
    uint8_t y = x>>4;
    if (y || lz) 
        *buf++ = hexletters[y];
    *buf++ = hexletters[x & 0xf];
    return buf;
}

inline char* ZeroXPrefix(char *buf, bool zx) {
    if (zx) {
        *buf++ = '0';
        *buf++ = 'x';
    }
    return buf;
}


std::string Hex(uint8_t x, bool lz, bool zx) {
    char buf[32] = { 0, };
    char *b = ZeroXPrefix(buf, zx);
    b = HexHelper(b, x, lz);
    return std::move(std::string(buf));
}
std::string Hex(uint16_t x, bool lz, bool zx) {
    char buf[32] = { 0, };
    char *b = ZeroXPrefix(buf, zx);
    b = HexHelper(b, x>>8, lz);
    b = HexHelper(b, x, true);
    return std::move(std::string(buf));
}
std::string Hex(uint32_t x, bool lz, bool zx) {
    char buf[32] = { 0, };
    char *b = ZeroXPrefix(buf, zx);
    b = HexHelper(b, x>>24, lz);
    b = HexHelper(b, x>>16, true);
    b = HexHelper(b, x>>8, true);
    b = HexHelper(b, x, true);
    return std::move(std::string(buf));
}
std::string Hex(uint64_t x, bool lz, bool zx) {
    char buf[32] = { 0, };
    char *b = ZeroXPrefix(buf, zx);
    b = HexHelper(b, x>>56, lz);
    b = HexHelper(b, x>>48, true);
    b = HexHelper(b, x>>40, true);
    b = HexHelper(b, x>>32, true);
    b = HexHelper(b, x>>24, true);
    b = HexHelper(b, x>>16, true);
    b = HexHelper(b, x>>8, true);
    b = HexHelper(b, x, true);
    return std::move(std::string(buf));
}

#if 0
template<typename ...Args>
void Log(LogLevel level, Args ...args) {
    if (!logging_init_done)
        logging_init();

    if (level > loglevel)
        return;

    const char* color = "";
    const char* prefix = "[?]";
    switch(level) {
        case LL_FATAL:
            prefix = "[F]";
            color = RED;
            break;
        case LL_ERROR:
            prefix = "[E]";
            color = RED;
            break;
        case LL_WARN:
            prefix = "[W]";
            color = YELLOW;
            break;
        case LL_INFO:
            prefix = "[I]";
            color = CYAN;
            break;
        case LL_VERBOSE:
            prefix = "[V]";
            color = GREEN;
            break;
        default:
            ; // Do nothing
    }

    if (logfp_isatty) {
        Print(logfp, BOLD, color, prefix, args..., RESET, "\n");
    } else {
        Println(logfp, prefix, args...);
    }
    if (level == LL_FATAL) {
        abort();
    }
}
#endif

void LogF(LogLevel level, const char *fmt, ...) {
    if (level > loglevel)
        return;

    char buf[1024];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, sizeof(buf)-1, fmt, ap);
    va_end(ap);
    Log(level, buf);
}

}  // namespace
