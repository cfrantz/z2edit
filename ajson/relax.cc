#include "ajson/relax.h"

#include <cctype>
#include <string>

#include "ajson/document.h"
#include "util/status.h"

namespace ajson {
bool isidchar(int ch) {
    return isalnum(ch) || isdigit(ch) || ch == '_' || ch == '$';
}

absl::Status Relax::error(std::string_view message, Location& loc) {
    return absl::InvalidArgumentError(
        absl::StrCat(message, " at ", loc.location()));
}

absl::StatusOr<char> Relax::get(std::istream& in, Location& loc) {
    auto ch = in.get();
    if (ch == std::char_traits<char>::eof()) {
        return absl::InternalError(
            absl::StrCat("unexpected end of file at ", loc.location()));
    }
    if (ch == '\n') {
        loc.column = 0;
        loc.line += 1;
    } else {
        loc.column += 1;
    }
    return ch;
}

absl::Status Relax::consume_space(std::istream& in, Location& loc,
                                  bool* observed_eol) {
    while (isspace(in.peek())) {
        ASSIGN_OR_RETURN(auto ch, get(in, loc));
        if (ch == '\n' && observed_eol) *observed_eol = true;
    }
    return absl::OkStatus();
}

Relax::DocStatus Relax::parse(std::istream& in) {
    Location loc;
    ASSIGN_OR_RETURN(auto doc, parse_value(in, loc));
    Location trailer_loc = loc;
    ASSIGN_OR_RETURN(auto trailer, parse_value(in, loc));
    if (!doc) return nullptr;
    if (trailer) {
        if (trailer->has_value()) {
            return error("unexpected value after document", trailer_loc);
        }
        if (doc->type() != document::Type::Fragment) {
            doc = Document::Fragment(doc);
        }
        doc->extend(trailer).IgnoreError();
    }
    LOG(INFO) << "returning type=" << int(doc->type()) << " @" << doc.get();
    return doc;
}

absl::StatusOr<std::shared_ptr<Document>> Relax::parse(const std::string& str) {
    std::stringstream in(str);
    return parse(in);
}

Relax::DocStatus Relax::parse_value(std::istream& in, Location& loc,
                                    char terminator) {
    int ch;
    std::vector<std::shared_ptr<Document>> doc;
    RETURN_IF_ERROR(consume_space(in, loc));
    Location start = loc;
    for (;;) {
        std::shared_ptr<Document> node;
        ch = in.peek();
        if (ch == std::char_traits<char>::eof()) {
            // Nothing to do
            break;
        } else if (terminator && ch == terminator) {
            break;
        } else if (ch == '#') {
            ASSIGN_OR_RETURN(node, parse_hash_comment(in, loc));
            doc.push_back(node);
            // A comment isn't a value.
            continue;
        } else if (ch == '/') {
            ASSIGN_OR_RETURN(node, parse_slash_comment(in, loc));
            doc.push_back(node);
            // A comment isn't a value.
            continue;
        } else if (isdigit(ch) || ch == '.' || ch == '+' || ch == '-') {
            ASSIGN_OR_RETURN(node, parse_number(in, loc));
        } else if (ch == '"' || ch == '\'') {
            ASSIGN_OR_RETURN(node, parse_quoted(in, loc));
        } else if (ch == '[') {
            ASSIGN_OR_RETURN(node, parse_sequence(in, loc));
        } else if (ch == '{') {
            ASSIGN_OR_RETURN(node, parse_mapping(in, loc));
        } else if (isidchar(ch)) {
            ASSIGN_OR_RETURN(
                node, parse_barestring(in, loc, terminator == '}' ? ":}" : ""));
        } else {
            char buf[2] = {char(ch), 0};
            return error(absl::StrCat("unexpected character '", buf, "'"), loc);
        }
        doc.push_back(node);
        break;
    }
    if (doc.size() == 0) {
        return nullptr;
    } else if (doc.size() == 1) {
        return doc[0];
    } else {
        return std::make_shared<Document>(
            document::Fragment{std::move(doc), start});
    }
}

Relax::DocStatus Relax::parse_number(std::istream& in, Location& loc) {
    auto start = loc;
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    bool negative;
    if (ch == '+' || ch == '-') {
        if (ch == '+' && !options_.number_plus) {
            return error("'+' not allowed", start);
        }
        if (ch == '-') negative = true;
        ASSIGN_OR_RETURN(ch, get(in, loc));
    }
    if (ch == '0') {
        if (in.peek() == '.') {
            in.unget();
            loc.column -= 1;
            return parse_decimal(in, negative, start, loc);
        }
        return parse_integer(in, negative, start, loc);
    } else {
        in.unget();
        loc.column -= 1;
        return parse_decimal(in, negative, start, loc);
    }
}

Relax::DocStatus Relax::parse_decimal(std::istream& in, bool negative,
                                      Location& start, Location& loc) {
    std::string buf;
    Location dot;
    for (;;) {
        int next = tolower(in.peek());
        if (isdigit(next) || isalnum(next) || next == '.' || next == '+' ||
            next == '-') {
            if (next == '.') dot = loc;
            auto status = get(in, loc);
            if (!status.ok()) return status.status();
            buf.push_back(next);
        } else {
            break;
        }
    }
    if (buf.find_first_of(".ein") != std::string::npos) {
        double real;
        if (buf[0] == '.' && !options_.number_lax_decimal) {
            return error("'.' not allowed", dot);
        }
        if (absl::SimpleAtod(buf, &real)) {
            if (negative) real = -real;
            return Document::Real(real, start);
        } else {
            return error("malformed number", start);
        }

    } else {
        uint64_t value;
        if (absl::SimpleAtoi(buf, &value)) {
            return Document::Int(value, negative, Base::Decimal, 0, start);
        } else {
            return error("malformed number", start);
        }
    }
}

Relax::DocStatus Relax::parse_integer(std::istream& in, bool negative,
                                      Location& start, Location& loc) {
    uint32_t base = 8, shift = 3;
    uint64_t value = 0;
    int next = toupper(in.peek());
    switch (next) {
        case 'B':
            base = 2;
            shift = 1;
            get(in, loc).IgnoreError();
            break;
        case 'O':
            base = 8;
            shift = 3;
            get(in, loc).IgnoreError();
            break;
        case 'X':
            base = 16;
            shift = 4;
            get(in, loc).IgnoreError();
            break;
        default:
            /* nothing */;
    }
    if (base == 2 && !options_.number_bin)
        return error("binary literals not allowed", start);
    if (base == 16 && !options_.number_hex)
        return error("hex literals not allowed", start);
    for (;;) {
        next = toupper(in.peek());
        if (!isxdigit(next)) break;
        auto status = get(in, loc);
        if (!status.ok()) return status.status();
        if (base == 2 && !(next >= '0' && next <= '1')) {
            return error("bad digit for binary literal", loc);
        } else if (base == 8 && !(next >= '0' && next <= '7')) {
            return error("bad digit for binary literal", loc);
        }
        value <<= shift;
        value |= next < 'A' ? next - '0' : next - 'A' + 10;
    }
    if (base == 8 && value != 0 && !options_.number_oct)
        return error("octal literals not allowed", start);
    return Document::Int(value, negative, Base(base), 0, start);
}

Relax::DocStatus Relax::parse_quoted(std::istream& in, Location& loc) {
    auto start = loc;
    int next = in.peek();
    if (next == '\'' && !options_.string_single_quote)
        return error("single quote not allowed", loc);
    ASSIGN_OR_RETURN(char quote, get(in, loc));

    char ch;
    bool triple_quoted = false;
    bool multiline = false;
    next = in.peek();
    if (next == quote) {
        get(in, loc).IgnoreError();
        int next = in.peek();
        if (next == quote) {
            get(in, loc).IgnoreError();
            if (!options_.string_hjson_multiline)
                return error("triple quoted string not allowed", start);
            triple_quoted = true;
            multiline = true;
        } else {
            return Document::String("", StringFormat::None, start);
        }
    }

    std::string buf;
    for (;;) {
        next = in.peek();
        if (next == quote) {
            // One quote, possible end of string.
            ASSIGN_OR_RETURN(auto q1, get(in, loc));
            next = in.peek();
            if (!triple_quoted && next != quote) {
                // End of string.
                break;
            }
            // Second quote character and we're in a triple quoted string.
            // Get the character and hold onto it.
            ASSIGN_OR_RETURN(auto q2, get(in, loc));
            // Examine third character.
            next = in.peek();
            if (next == quote) {
                // Is a quote, end of triple quoted string.
                // Get the final quote.
                ASSIGN_OR_RETURN(ch, get(in, loc));
                break;
            } else {
                // Not a third quote; keep the previous chars
                buf.push_back(q1);
                buf.push_back(q2);
            }
        }
        ASSIGN_OR_RETURN(ch, get(in, loc));
        if (ch == '\\') {
            ASSIGN_OR_RETURN(ch, get(in, loc));
            switch (ch) {
                case 'b':
                    buf.push_back('\b');
                    break;
                case 't':
                    buf.push_back('\t');
                    break;
                case 'n':
                    buf.push_back('\n');
                    break;
                case 'f':
                    buf.push_back('\f');
                    break;
                case 'r':
                    buf.push_back('\r');
                    break;
                case '"':
                    buf.push_back('\"');
                    break;
                case '\'':
                    buf.push_back('\'');
                    break;
                case '\\':
                    buf.push_back('\\');
                    break;
                case '/':
                    buf.push_back('/');
                    break;
                case '\n':
                    if (!options_.string_json5_multiline)
                        return error("json5 multiline strings not allowed",
                                     loc);
                    buf.push_back('\n');
                    multiline = true;
                    break;
                case 'u': {
                    std::string hex;
                    for (size_t i = 0; i < 4; ++i) {
                        next = in.peek();
                        if (!isxdigit(next)) break;
                        ASSIGN_OR_RETURN(ch, get(in, loc));
                        hex.push_back(ch);
                    }
                    if (hex.empty())
                        return error("invalid unicode escape sequence", loc);
                    // Encode the codepoint as utf-8.
                    uint32_t u = std::stoul(hex, nullptr, 16);
                    if (u < 0x80) {
                        buf.push_back(char(u));
                    } else if (u < 0x800) {
                        buf.push_back(char(0xC0 | ((u >> 6) & 0x1f)));
                        buf.push_back(char(0x80 | (u & 0x3f)));
                    } else if (u < 0x10000) {
                        buf.push_back(char(0xE0 | ((u >> 12) & 0x0f)));
                        buf.push_back(char(0x80 | ((u >> 6) & 0x3f)));
                        buf.push_back(char(0x80 | (u & 0x3f)));
                    } else {
                        buf.push_back(char(0xF0 | ((u >> 18) & 0x07)));
                        buf.push_back(char(0x80 | ((u >> 12) & 0x3f)));
                        buf.push_back(char(0x80 | ((u >> 6) & 0x3f)));
                        buf.push_back(char(0x80 | (u & 0x3f)));
                    }
                } break;
                default:
                    return error("unknown escape sequence", loc);
            }
            continue;
        }
        buf.push_back(ch);
    }
    // TODO: de-dent triple quoted strings.
    return Document::String(
        buf, multiline ? StringFormat::Block : StringFormat::None, start);
}

Relax::DocStatus Relax::parse_barestring(std::istream& in, Location& loc,
                                         std::string_view terminator) {
    std::string buf;
    Location start = loc;
    for (;;) {
        int next = in.peek();
        // FIXME: handle kvpair keys and bare strings here.
        if (!isidchar(next)) {
            if (buf == "null") {
                return Document::Null(loc);
            } else if (buf == "true") {
                return Document::Boolean(true, loc);
            } else if (buf == "false") {
                return Document::Boolean(false, loc);
            } else if (buf == "Infinity") {
                return Document::Real(std::numeric_limits<double>::infinity(),
                                      loc);
            } else if (buf == "NaN") {
                return Document::Real(std::nan(""), loc);
            } else {
                // Fixme: error
            }
        }
        if (terminator.find_first_of(char(next)) != std::string_view::npos)
            break;
        if (next == '\n' || next == std::char_traits<char>::eof()) break;
        ASSIGN_OR_RETURN(char ch, get(in, loc));
        buf.push_back(ch);
    }
    if (!options_.string_unquoted)
        return error("unquoted strings not allowed", start);
    return Document::String(buf, StringFormat::Unquoted, start);
}

Relax::DocStatus Relax::parse_hash_comment(std::istream& in, Location& loc,
                                           bool* observed_eol) {
    auto start = loc;
    if (!options_.comment_hash) {
        return error("hash comment not allowed", start);
    }
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    bool eat_space = false;
    std::string buf;
    int next;
    for (;;) {
        next = in.peek();
        if (eat_space) {
            if (next == ' ' || next == '\t') {
                get(in, loc).IgnoreError();
                continue;
            } else if (next == '#') {
                get(in, loc).IgnoreError();
                eat_space = false;
                continue;
            } else {
                break;
            }
        } else {
            if (next == std::char_traits<char>::eof()) break;
            ASSIGN_OR_RETURN(ch, get(in, loc));
            buf.push_back(ch);
            if (next == '\n') {
                eat_space = true;
            }
        }
    }
    if (observed_eol) *observed_eol = true;
    if (next != std::char_traits<char>::eof()) buf.pop_back();
    return Document::Comment(buf, CommentFormat::Hash, start);
}

Relax::DocStatus Relax::parse_slash_comment(std::istream& in, Location& loc,
                                            bool* observed_eol) {
    auto start = loc;
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    int next = in.peek();
    if (next == '*') {
        in.unget();
        return parse_block_comment(in, loc, observed_eol);
    } else if (next != '/') {
        return error("invalid comment", start);
    }
    get(in, loc).IgnoreError();
    if (!options_.comment_hash) {
        return error("slash comment not allowed", start);
    }

    bool eat_space = false;
    std::string buf;
    for (;;) {
        next = in.peek();
        if (eat_space) {
            if (next == ' ' || next == '\t') {
                get(in, loc).IgnoreError();
                continue;
            } else if (next == '/') {
                get(in, loc).IgnoreError();
                next = in.peek();
                if (next != '/') {
                    in.unget();
                    break;
                }
                eat_space = false;
                continue;
            } else {
                break;
            }
        } else {
            if (next == std::char_traits<char>::eof()) break;
            ASSIGN_OR_RETURN(ch, get(in, loc));
            buf.push_back(ch);
            if (next == '\n') {
                eat_space = true;
            }
        }
    }
    if (observed_eol) *observed_eol = true;
    if (next != std::char_traits<char>::eof()) buf.pop_back();
    return Document::Comment(buf, CommentFormat::SlashSlash, start);
}

Relax::DocStatus Relax::parse_block_comment(std::istream& in, Location& loc,
                                            bool* observed_eol) {
    auto start = loc;
    std::string buf;
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    int next = in.peek();
    if (next != '*') {
        return error("invalid comment", start);
    }
    if (!options_.comment_block) {
        return error("block comment not allowed", start);
    }
    buf.push_back(ch);
    ASSIGN_OR_RETURN(ch, get(in, loc));
    buf.push_back(ch);

    for (;;) {
        ASSIGN_OR_RETURN(ch, get(in, loc));
        if (ch == '*') {
            next = in.peek();
            if (next == '/') {
                buf.push_back(ch);
                ASSIGN_OR_RETURN(ch, get(in, loc));
                buf.push_back(ch);
                break;
            }
        }
        buf.push_back(ch);
    }
    RETURN_IF_ERROR(consume_space(in, loc, observed_eol));
    // TODO: post-process buf.
    return Document::Comment(buf, CommentFormat::Block, start);
}

Relax::DocStatus Relax::parse_sequence(std::istream& in, Location& loc) {
    auto doc = Document::Sequence({}, loc);
    // Consume the opening bracket.
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    bool eol = false, comma = false;
    for (;;) {
        ASSIGN_OR_RETURN(auto item, parse_value(in, loc, ']'));
        if (item == nullptr) break;
        // Look for a comma or newline, possibly with comments.
        // If we observe either a comma or newline, then we are ready for the
        // next item in the list (subject to comma settings in options_).
        eol = false;
        comma = false;
        int next;
        for (;;) {
            next = in.peek();
            if (isspace(next)) {
                ASSIGN_OR_RETURN(ch, get(in, loc));
                if (ch == '\n') eol = true;
            } else if (next == '#' || next == '/') {
                if (eol) break;
                std::shared_ptr<Document> comment;
                if (next == '#') {
                    ASSIGN_OR_RETURN(comment,
                                     parse_hash_comment(in, loc, &eol));
                } else {
                    ASSIGN_OR_RETURN(comment,
                                     parse_slash_comment(in, loc, &eol));
                }
                if (item->type() != document::Type::Fragment) {
                    item = Document::Fragment(item, comment);
                } else {
                    item->append(comment).IgnoreError();
                }
            } else if (next == ',') {
                get(in, loc).IgnoreError();
                comma = true;
            } else {
                break;
            }
        }
        if (item->has_value() && next != ']') {
            if (!comma && !options_.comma_optional) {
                return error("expecting ','", loc);
            }
            if (!comma && !eol) {
                return error("expecting ',' or newline", loc);
            }
        }
        doc->append(item).IgnoreError();
    }
    // Consume the closing bracket.
    get(in, loc).IgnoreError();
    if (comma && !options_.comma_trailing) {
        return error("comma not allowed", loc);
    }
    return doc;
}

Relax::DocStatus Relax::parse_mapping(std::istream& in, Location& loc) {
    auto doc = Document::Mapping({}, loc);
    // Consume the opening curly brace.
    ASSIGN_OR_RETURN(char ch, get(in, loc));
    RETURN_IF_ERROR(consume_space(in, loc));
    bool eol = false, comma = false;
    for (;;) {
        auto kvpair = Document::Fragment({}, loc);
        ASSIGN_OR_RETURN(auto item, parse_value(in, loc, '}'));
        if (item == nullptr) break;
        if (!item->has_value()) {
            // Handle an empty mapping with a coments inside.
            doc->append(item).IgnoreError();
            continue;
        }
        kvpair->extend(item).IgnoreError();
        ASSIGN_OR_RETURN(auto colon, parse_value(in, loc, ':'));
        if (colon) {
            // TODO: check that `colon` contains no content
            kvpair->extend(colon).IgnoreError();
        }
        ASSIGN_OR_RETURN(ch, get(in, loc));
        if (ch != ':') {
            return error("expecting ':'", loc);
        }
        // FIXME: does hjson allow "{foo: barestring}" ?
        ASSIGN_OR_RETURN(auto value, parse_value(in, loc, '}'));
        if (value == nullptr) {
            return error("expecting value", loc);
        }
        kvpair->extend(value).IgnoreError();

        // Look for a comma or newline, possibly with comments.
        // If we observe either a comma or newline, then we are ready for the
        // next item in the list (subject to comma settings in options_).
        eol = false;
        comma = false;
        int next;
        for (;;) {
            next = in.peek();
            if (isspace(next)) {
                ASSIGN_OR_RETURN(ch, get(in, loc));
                if (ch == '\n') eol = true;
            } else if (next == '#' || next == '/') {
                if (eol) break;
                std::shared_ptr<Document> comment;
                if (next == '#') {
                    ASSIGN_OR_RETURN(comment,
                                     parse_hash_comment(in, loc, &eol));
                } else {
                    ASSIGN_OR_RETURN(comment,
                                     parse_slash_comment(in, loc, &eol));
                }
                kvpair->extend(comment).IgnoreError();
            } else if (next == ',') {
                get(in, loc).IgnoreError();
                comma = true;
            } else {
                break;
            }
        }
        if (kvpair->has_value() && next != '}') {
            if (!comma && !options_.comma_optional) {
                return error("expecting ','", loc);
            }
            if (!comma && !eol) {
                return error("expecting ',' or newline", loc);
            }
        }
        doc->append(kvpair).IgnoreError();
    }
    // Consume the closing curly brace.
    get(in, loc).IgnoreError();
    if (comma && !options_.comma_trailing) {
        return error("comma not allowed", loc);
    }
    return doc;
}

}  // namespace ajson
