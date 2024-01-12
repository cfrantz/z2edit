#ifndef EMPTY_PROJECT_AJSON_RELAX_H
#define EMPTY_PROJECT_AJSON_RELAX_H

#include <iostream>
#include <string>

#include "absl/status/statusor.h"
#include "ajson/document.h"

namespace ajson {
class Relax {
  public:
    struct Options {
        bool comma_trailing = true;
        bool comma_optional = true;
        bool number_bin = true;
        bool number_hex = true;
        bool number_oct = true;
        bool number_plus = true;
        bool number_lax_decimal = true;
        bool string_single_quote = true;
        bool string_unquoted = true;
        bool string_ident = true;
        bool string_json5_multiline = true;
        bool string_hjson_multiline = true;
        bool comment_slash = true;
        bool comment_hash = true;
        bool comment_block = true;
    };
    Relax(Options options) : options_(options) {}
    Relax() : Relax(Options{}) {}

    absl::StatusOr<std::unique_ptr<Document>> parse(std::istream& in);
    absl::StatusOr<std::unique_ptr<Document>> parse(const std::string& str);

  private:
    typedef absl::StatusOr<std::unique_ptr<Document>> DocStatus;
    Options options_{};

    absl::Status error(std::string_view message, Location& loc);
    absl::StatusOr<char> get(std::istream& in, Location& loc);
    absl::Status consume_space(std::istream& in, Location& loc,
                               bool* observed_eol = nullptr);
    DocStatus parse_value(std::istream& in, Location& loc,
                          char terminator = '\0');
    DocStatus parse_number(std::istream& in, Location& loc);
    DocStatus parse_decimal(std::istream& in, bool negative, Location& start,
                            Location& loc);
    DocStatus parse_integer(std::istream& in, bool negative, Location& start,
                            Location& loc);
    DocStatus parse_barestring(std::istream& in, Location& loc,
                               std::string_view terminator);
    DocStatus parse_quoted(std::istream& in, Location& loc);
    DocStatus parse_hash_comment(std::istream& in, Location& loc,
                                 bool* observed_eol = nullptr);
    DocStatus parse_slash_comment(std::istream& in, Location& loc,
                                  bool* observed_eol = nullptr);
    DocStatus parse_block_comment(std::istream& in, Location& loc,
                                  bool* observed_eol = nullptr);
    DocStatus parse_sequence(std::istream& in, Location& loc);
    DocStatus parse_mapping(std::istream& in, Location& loc);
};
}  // namespace ajson

#endif  // EMPTY_PROJECT_AJSON_RELAX_H
