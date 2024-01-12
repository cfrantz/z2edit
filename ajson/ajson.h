#ifndef EMPTY_PROJECT_AJSON_AJSON_H
#define EMPTY_PROJECT_AJSON_AJSON_H
#include <iostream>
#include <set>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "ajson/document.h"
#include "ajson/reflect.h"

namespace ajson {

enum class Multiline : uint8_t {
    None,
    Json5,
    Hjson,
};

struct JsonEncoder {
  public:
    struct Options {
        uint32_t indent = 4;
        bool bare_keys = false;
        Multiline multiline = Multiline::None;
        CommentFormat comfmt = CommentFormat::None;
        bool strict_numeric_limits = true;
        std::set<Base> allowed_bases{};
        std::set<Base> literal_bases{};
        std::set<CommentFormat> allowed_comfmt{};
    };
    JsonEncoder() = default;
    JsonEncoder(Options options) : options_(options) {}

    static JsonEncoder Json5() {
        return JsonEncoder(Options{
            .bare_keys = true,
            .multiline = Multiline::Json5,
            .comfmt = CommentFormat::SlashSlash,
            .literal_bases{Base::Hex},
        });
    }

    static JsonEncoder Hjson() {
        return JsonEncoder(Options{
            .bare_keys = true,
            .multiline = Multiline::Hjson,
            .comfmt = CommentFormat::SlashSlash,
            .allowed_comfmt{
                CommentFormat::Block,
                CommentFormat::Hash,
                CommentFormat::SlashSlash,
            },
        });
    }

    absl::Status encode(std::ostream& out, const Document* doc);
    absl::StatusOr<std::string> encode(const Document* doc);

    // FIXME: Reflection should be const.
    // Json encoding is read-only... promise.
    absl::Status encode(std::ostream& out, Reflection& r);
    absl::StatusOr<std::string> encode(Reflection& r);

  private:
    absl::Status emit_comment(std::ostream& out, const document::Comment& doc);
    absl::Status emit_string(std::ostream& out, const document::String& doc);
    absl::Status emit_string_strict(std::ostream& out, std::string_view value);
    absl::Status emit_string_multiline(std::ostream& out,
                                       std::string_view value);
    absl::Status emit_boolean(std::ostream& out, const document::Boolean& doc);
    absl::Status emit_int(std::ostream& out, const document::Int& doc);
    absl::Status emit_real(std::ostream& out, const document::Real& doc);
    absl::Status emit_mapping(std::ostream& out, const document::Mapping& doc);
    absl::Status emit_sequence(std::ostream& out,
                               const document::Sequence& doc);
    absl::Status emit_bytes(std::ostream& out, const document::Bytes& doc);
    absl::Status emit_null(std::ostream& out, const document::Null& doc);
    absl::Status emit_compact(std::ostream& out, const document::Compact& doc);
    absl::Status emit_fragment(std::ostream& out,
                               const document::Fragment& doc);

    template <typename T>
    bool set_has(const std::set<T>& set, const T& elem) const {
        auto search = set.find(elem);
        return search != set.end();
    }

    void write(std::ostream& out, std::string_view s) {
        out.write(s.data(), s.size());
        column_ += s.size();
    }
    void writeln(std::ostream& out) {
        if (compact_) return;
        out.write("\n", 1);
        column_ = 0;
        line_ += 1;
    }
    void space(std::ostream& out, size_t n = 1) {
        if (compact_) return;
        while (n) {
            size_t len = std::min(n, space_.size());
            write(out, space_.substr(0, len));
            n -= len;
        }
    }
    void indent(std::ostream& out, size_t extra = 0) {
        if (compact_) return;
        space(out, indent_ + extra);
    }

    size_t indent_ = 0;
    uint32_t line_ = 0;
    uint32_t column_ = 0;
    bool compact_ = false;
    bool bareword_ = false;
    Options options_;
    // Upto 80 chars of space to use for indenting.
    static constexpr inline absl::string_view space_ =
        "                                                                      "
        "          ";
};

}  // namespace ajson

#endif  // EMPTY_PROJECT_AJSON_AJSON_H
