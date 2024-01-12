#include "ajson/ajson.h"

#include <sstream>

#include "absl/status/status.h"
#include "ajson/document.h"
#include "ajson/reflect.h"
#include "ajson/serialize.h"
#include "util/status.h"

namespace ajson {
namespace internal {
std::string print_integer(uint64_t val, Base base, uint32_t width,
                          bool signifier = true) {
    uint32_t shift;
    uint32_t mask;
    char prefix;
    switch (base) {
        case Base::Binary:
            shift = 1;
            mask = 1;
            prefix = 'b';
            break;
        case Base::Octal:
            shift = 3;
            mask = 7;
            prefix = 'o';
            break;
        case Base::Hex:
            shift = 4;
            mask = 15;
            prefix = 'x';
            break;
        default:
            return absl::StrCat(val);
    }
    const char hex[] = "0123456789ABCDEF";
    char buf[68];
    char* b = buf + sizeof(buf);
    uint32_t n = 0;
    *--b = 0;
    b[-1] = '0';
    while (val) {
        *--b = hex[val & mask];
        val >>= shift;
        n += 1;
    }
    while (n < width) {
        *--b = '0';
    }
    if (signifier) {
        *--b = prefix;
        *--b = '0';
    }
    return std::string(b);
}

constexpr char BB = 'b';
constexpr char TT = 't';
constexpr char NN = 'n';
constexpr char FF = 'f';
constexpr char RR = 'r';
constexpr char QU = '"';
constexpr char BS = '\\';
constexpr char UU = 'u';
constexpr char __ = 0;
constexpr char ESCAPE[256] = {
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    UU, UU, UU, UU, UU, UU, UU, UU, BB, TT, NN, UU, FF, RR, UU, UU,  // 0
    UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU, UU,  // 1
    __, __, QU, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 2
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 3
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 4
    __, __, __, __, __, __, __, __, __, __, __, __, BS, __, __, __,  // 5
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 6
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 7
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 8
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // 9
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // A
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // B
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // C
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // D
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // E
    __, __, __, __, __, __, __, __, __, __, __, __, __, __, __, __,  // F
};

bool is_identifier(char ch) {
    switch (ch) {
        case '0' ... '9':
            return true;
        case 'A' ... 'Z':
            return true;
        case 'a' ... 'z':
            return true;
        case '_':
            return true;
        case '$':
            return true;
        default:
            return false;
    }
}

bool is_reserved_word(std::string_view word) {
    const std::set<std::string_view> reserved = {
        "break",   "do",        "instanceof", "typeof", "case",
        "else",    "new",       "var",        "catch",  "finally",
        "return",  "void",      "continue",   "for",    "switch",
        "while",   "debugger",  "function",   "this",   "with",
        "default", "if",        "throw",      "",       "delete",
        "in",      "try",       "class",      "enum",   "extends",
        "super",   "const",     "export",     "import", "implements",
        "let",     "private",   "public",     "yield",  "interface",
        "package", "protected", "static",     "null",   "true",
        "false",
    };
    return reserved.find(word) != reserved.end();
}

bool is_legal_bareword(std::string_view word) {
    if (word.empty()) return false;
    return !((word[0] >= '0' && word[0] <= '9') ||
             word.find_first_not_of("$0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcde"
                                    "fghijklmnopqrstuvwxyz_") !=
                 std::string_view::npos ||
             is_reserved_word(word));
}

}  // namespace internal

absl::Status JsonEncoder::encode(std::ostream& out, const Document* doc) {
    switch (doc->type()) {
        case document::Type::Comment:
            return emit_comment(out, std::get<document::Comment>(doc->node));
        case document::Type::String:
            return emit_string(out, std::get<document::String>(doc->node));
        case document::Type::Boolean:
            return emit_boolean(out, std::get<document::Boolean>(doc->node));
        case document::Type::Int:
            return emit_int(out, std::get<document::Int>(doc->node));
        case document::Type::Real:
            return emit_real(out, std::get<document::Real>(doc->node));
        case document::Type::Null:
            return emit_null(out, std::get<document::Null>(doc->node));
        case document::Type::Bytes:
            return emit_bytes(out, std::get<document::Bytes>(doc->node));
        case document::Type::Mapping:
            return emit_mapping(out, std::get<document::Mapping>(doc->node));
        case document::Type::Sequence:
            return emit_sequence(out, std::get<document::Sequence>(doc->node));
        case document::Type::Compact:
            return emit_compact(out, std::get<document::Compact>(doc->node));
        case document::Type::Fragment:
            return emit_fragment(out, std::get<document::Fragment>(doc->node));
    }
    return absl::UnknownError("bad document");
}

absl::Status JsonEncoder::encode(std::ostream& out, Reflection& r) {
    Serializer s;
    ASSIGN_OR_RETURN(auto doc, s.serialize(r._ref()));
    return encode(out, doc.get());
}

absl::StatusOr<std::string> JsonEncoder::encode(const Document* doc) {
    std::stringstream buffer;
    RETURN_IF_ERROR(encode(buffer, doc));
    return buffer.str();
}

absl::StatusOr<std::string> JsonEncoder::encode(Reflection& r) {
    Serializer s;
    ASSIGN_OR_RETURN(auto doc, s.serialize(r._ref()));
    return encode(doc.get());
}

absl::Status JsonEncoder::emit_comment(std::ostream& out,
                                       const document::Comment& doc) {
    if (!compact_) {
        write(out, "# ");
        write(out, doc.value);
    }
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_string(std::ostream& out,
                                      const document::String& doc) {
    if (options_.multiline != Multiline::None &&
        doc.format == StringFormat::Block) {
        return emit_string_multiline(out, doc.value);
    } else {
        return emit_string_strict(out, doc.value);
    }
}

absl::Status JsonEncoder::emit_string_strict(std::ostream& out,
                                             std::string_view value) {
    if (bareword_ && internal::is_legal_bareword(value)) {
        write(out, value);
        return absl::OkStatus();
    }
    size_t start = 0;
    write(out, "\"");
    for (size_t i = 0; i < value.size(); ++i) {
        char escape = internal::ESCAPE[size_t(value[i])];
        if (!escape) continue;
        if (start < i) write(out, value.substr(start, i - start));
        switch (escape) {
            case internal::UU:
                write(out, "\\u");
                write(out, internal::print_integer(value[i], Base::Hex, 4,
                                                   /*signifier=*/false));
                break;
            default:
                char buf[] = {'\\', escape, 0};
                write(out, buf);
                break;
        }
        start = i + 1;
    }
    if (start < value.size()) write(out, value.substr(start));
    write(out, "\"");
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_string_multiline(std::ostream& out,
                                                std::string_view value) {
    size_t start = 0;
    if (options_.multiline == Multiline::Hjson) {
        indent_ += options_.indent;
        writeln(out);
        indent(out);
        write(out, "'''");
        writeln(out);
        indent(out);
    } else {
        write(out, "\"");
    }
    for (size_t i = 0; i < value.size(); ++i) {
        char escape = internal::ESCAPE[size_t(value[i])];
        if (!escape) continue;
        if (start < i) write(out, value.substr(start, i - start));
        switch (escape) {
            case internal::UU:
                write(out, "\\u");
                write(out, internal::print_integer(value[i], Base::Hex, 4,
                                                   /*signifier=*/false));
                break;
            case internal::NN:
                switch (options_.multiline) {
                    case Multiline::None:
                        write(out, "\\n");
                        break;
                    case Multiline::Json5:
                        write(out, "\\\n");
                        break;
                    case Multiline::Hjson:
                        writeln(out);
                        indent(out);
                        break;
                }
                break;
            default:
                char buf[] = {escape, 0};
                write(out, absl::StrCat("\\", buf));
                break;
        }
        start = i + 1;
    }
    if (start < value.size()) write(out, value.substr(start));
    if (options_.multiline == Multiline::Hjson) {
        writeln(out);
        indent(out);
        write(out, "'''");
        indent_ -= options_.indent;
    } else {
        write(out, "\"");
    }
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_boolean(std::ostream& out,
                                       const document::Boolean& doc) {
    write(out, doc.value ? "true" : "false");
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_int(std::ostream& out,
                                   const document::Int& doc) {
    Base base = Base::Decimal;
    bool lit = true;
    if (set_has(options_.allowed_bases, doc.base)) {
        base = doc.base;
        lit = false;
    }
    if (set_has(options_.literal_bases, doc.base)) {
        base = doc.base;
        lit = true;
    }

    if (!lit) write(out, "\"");
    switch (base) {
        case Base::Decimal:
            if (doc.negative) write(out, "-");
            write(out, absl::StrCat(doc.value));
            break;
        case Base::Binary:
            write(out, internal::print_integer(doc.value, base, doc.size * 8));
            break;
        case Base::Octal:
            write(out, internal::print_integer(doc.value, base,
                                               (doc.size * 8 + 2) / 3));
            break;
        case Base::Hex:
            write(out, internal::print_integer(doc.value, base, doc.size * 2));
            break;
    }
    if (!lit) write(out, "\"");
    return absl::OkStatus();
}
absl::Status JsonEncoder::emit_real(std::ostream& out,
                                    const document::Real& doc) {
    write(out, absl::StrCat(doc.value));
    return absl::OkStatus();
}
absl::Status JsonEncoder::emit_mapping(std::ostream& out,
                                       const document::Mapping& doc) {
    write(out, "{");
    indent_ += options_.indent;
    size_t n = 0;
    size_t last = doc.value.size();
    for (const auto& entry : doc.value) {
        writeln(out);
        indent(out);
        n += 1;
        int i = 0;
        // Some formats allow barewords for keys. The string emitter will not
        // emit a bareword if the format doesn't allow it.
        bareword_ = true;
        for (const auto& item :
             std::get<document::Fragment>(entry->node).value) {
            if (i) space(out);
            if (item->has_value()) i++;
            RETURN_IF_ERROR(encode(out, item.get()));
            if (i < 2 && !item->has_value()) {
                writeln(out);
                indent(out);
            }
            if (i == 1) {
                bareword_ = false;
                write(out, ":");
            }
            if (i == 2 && n < last) write(out, ",");
        }
    }
    writeln(out);
    indent_ -= options_.indent;
    indent(out);
    write(out, "}");
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_sequence(std::ostream& out,
                                        const document::Sequence& doc) {
    write(out, "[");
    indent_ += options_.indent;
    size_t n = 0;
    size_t last = doc.value.size();
    for (const auto& entry : doc.value) {
        writeln(out);
        indent(out);
        n += 1;
        if (entry->type() != document::Type::Fragment) {
            RETURN_IF_ERROR(encode(out, entry.get()));
            if (n < last) write(out, ",");
            continue;
        }
        int i = 0;
        for (const auto& item :
             std::get<document::Fragment>(entry->node).value) {
            if (i) space(out);
            if (item->has_value()) i++;
            RETURN_IF_ERROR(encode(out, item.get()));
            if (i < 2 && !item->has_value()) {
                writeln(out);
                indent(out);
            }
            if (i == 1) write(out, ":");
            if (i == 2 && n < last) write(out, ",");
        }
    }
    writeln(out);
    indent_ -= options_.indent;
    indent(out);
    write(out, "]");
    return absl::OkStatus();
}

absl::Status JsonEncoder::emit_bytes(std::ostream& out,
                                     const document::Bytes& doc) {
    return absl::UnimplementedError("bytes support not yet implemented");
}
absl::Status JsonEncoder::emit_null(std::ostream& out,
                                    const document::Null& doc) {
    write(out, "null");
    return absl::OkStatus();
}
absl::Status JsonEncoder::emit_compact(std::ostream& out,
                                       const document::Compact& doc) {
    bool compact = compact_;
    compact_ = true;
    for (const auto& value : doc.value) {
        RETURN_IF_ERROR(encode(out, value.get()));
    }
    compact_ = compact;
    return absl::OkStatus();
}
absl::Status JsonEncoder::emit_fragment(std::ostream& out,
                                        const document::Fragment& doc) {
    for (const auto& value : doc.value) {
        RETURN_IF_ERROR(encode(out, value.get()));
    }
    return absl::OkStatus();
}

}  // namespace ajson
