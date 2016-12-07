#include "util/logging.h"
#include <gflags/gflags.h>

DEFINE_int32(loglevel, 4, "Logging level");
DEFINE_string(logfile, "", "Log to file");

namespace logging {

const char RESET[] = "\033[0m";
const char BOLD[]  = "\033[1m";

const char BLACK[]   = "\033[30m";
const char RED[]     = "\033[31m";
const char GREEN[]   = "\033[32m";
const char YELLOW[]  = "\033[33m";
const char BLUE[]    = "\033[34m";
const char MAGENTA[] = "\033[35m";
const char CYAN[]    = "\033[36m";
const char WHITE[]   = "\033[37m";

int logging_init_done;
LogLevel loglevel;
FILE* logfp;
int logfp_isatty;

void logging_init() {
    if (logging_init_done)
        return;

    bool initerror = false;
    loglevel = LogLevel(FLAGS_loglevel);
    if (FLAGS_logfile.empty()) {
        logfp = stderr;
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

}  // namespace
