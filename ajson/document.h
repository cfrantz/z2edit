#ifndef EMPTY_PROJECT_AJSON_DOCUMENT_H
#define EMPTY_PROJECT_AJSON_DOCUMENT_H

#include <cstdint>
#include <memory>
#include <string>
#include <variant>
#include <vector>

#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "util/status.h"

namespace ajson {
class Document;
enum class CommentFormat : uint32_t {
    None,
    Block,
    Hash,
    SlashSlash,
};
enum class StringFormat : uint32_t {
    None = 0,
    Quoted = 1,
    Unquoted = 2,
    Block = 3,
};
enum class Base : uint8_t {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hex = 16,
};

static std::string _debug_string(Document* d);

struct Location {
    uint32_t line = 0;
    uint32_t column = 0;
    std::string location() const {
        return absl::StrCat(line + 1, ":", column + 1);
    }
};

namespace document {
enum class Type : uint32_t {
    Comment,
    String,
    Boolean,
    Int,
    Real,
    Mapping,
    Sequence,
    Bytes,
    Null,
    Compact,
    Fragment,
};

struct Comment {
    std::string value;
    CommentFormat format = CommentFormat::None;
    Location location{};
    std::string debug_string() {
        return absl::StrCat("Comment(\"", value, "\", ", format, ")");
    }
};

struct String {
    std::string value;
    StringFormat format = StringFormat::None;
    Location location{};
    std::string debug_string() {
        return absl::StrCat("String(\"", value, "\", ", format, ")");
    }
};

struct Boolean {
    bool value;
    Location location{};
    std::string debug_string() {
        return absl::StrCat("Boolean(", value ? "true" : "false", ")");
    }
};

struct Int {
    uint64_t value;
    bool negative = false;
    Base base = Base::Decimal;
    uint8_t size = 0;
    Location location{};
    std::string debug_string() {
        return absl::StrCat("Int(", negative ? "-" : "", value, ", ", base,
                            ")");
    }
    template <typename T>
    T as() const {
        T v = negative ? -value : value;
        return v;
    }
};

struct Real {
    double value;
    Location location{};
    std::string debug_string() { return absl::StrCat("Real(", value, ")"); }
};

struct Null {
    Location location{};
    std::string debug_string() { return std::string("Null()"); }
};

struct Bytes {
    std::vector<uint8_t> value;
    Location location{};
    std::string debug_string() { return std::string("Bytes(...)"); }
};

struct Mapping {
    std::vector<std::shared_ptr<Document>> value;
    Location location{};
    std::string debug_string() {
        std::string a = "Mapping(";
        size_t i = 0;
        for (const auto& v : value) {
            if (i++) absl::StrAppend(&a, ", ");
            absl::StrAppend(&a, _debug_string(v.get()));
        }
        absl::StrAppend(&a, ")");
        return a;
    }
};

struct Sequence {
    std::vector<std::shared_ptr<Document>> value;
    Location location{};
    std::string debug_string() {
        std::string a = "Sequence(";
        size_t i = 0;
        for (const auto& v : value) {
            if (i++) absl::StrAppend(&a, ", ");
            absl::StrAppend(&a, _debug_string(v.get()));
        }
        absl::StrAppend(&a, ")");
        return a;
    }
};

struct Compact {
    std::vector<std::shared_ptr<Document>> value;
    Location location{};
    std::string debug_string() {
        std::string a = "Compact(";
        size_t i = 0;
        for (const auto& v : value) {
            if (i++) absl::StrAppend(&a, ", ");
            absl::StrAppend(&a, _debug_string(v.get()));
        }
        absl::StrAppend(&a, ")");
        return a;
    }
};

struct Fragment {
    std::vector<std::shared_ptr<Document>> value;
    Location location{};
    std::string debug_string() {
        std::string a = "Fragment(";
        size_t i = 0;
        for (const auto& v : value) {
            if (i++) absl::StrAppend(&a, ", ");
            absl::StrAppend(&a, _debug_string(v.get()));
        }
        absl::StrAppend(&a, ")");
        return a;
    }
};

}  // namespace document

struct Document {
    // clang-format off
    typedef std::variant<
        document::Comment,
        document::String,
        document::Boolean,
        document::Int,
        document::Real,
        document::Null,
        document::Bytes,
        document::Mapping,
        document::Sequence,
        document::Compact,
        document::Fragment
    > Node;
    // clang-format on
    Node node;

    document::Type type() const {
        if (std::holds_alternative<document::Comment>(node)) {
            return document::Type::Comment;
        } else if (std::holds_alternative<document::String>(node)) {
            return document::Type::String;
        } else if (std::holds_alternative<document::Boolean>(node)) {
            return document::Type::Boolean;
        } else if (std::holds_alternative<document::Int>(node)) {
            return document::Type::Int;
        } else if (std::holds_alternative<document::Real>(node)) {
            return document::Type::Real;
        } else if (std::holds_alternative<document::Null>(node)) {
            return document::Type::Null;
        } else if (std::holds_alternative<document::Bytes>(node)) {
            return document::Type::Bytes;
        } else if (std::holds_alternative<document::Mapping>(node)) {
            return document::Type::Mapping;
        } else if (std::holds_alternative<document::Sequence>(node)) {
            return document::Type::Sequence;
        } else if (std::holds_alternative<document::Compact>(node)) {
            return document::Type::Compact;
        } else if (std::holds_alternative<document::Fragment>(node)) {
            return document::Type::Fragment;
        } else {
            LOG(FATAL) << "Invalid document state";
            return document::Type::Null;
        }
    }

    std::string_view type_name() const {
        const char* names[] = {
            "comment", "string", "boolean", "integer", "real",     "mapping",
            "list",    "bytes",  "null",    "compact", "fragment",
        };
        return names[static_cast<int>(type())];
    }

    template <typename T>
    const StatusOrRef<T> as() const {
        if (std::holds_alternative<T>(node)) {
            const T* doc = &std::get<T>(node);
            return doc;
        } else {
            return absl::InvalidArgumentError("bad document type");
        }
    }

    std::string structure(bool detailed = false) const {
        std::string buf;
        if (std::holds_alternative<document::Comment>(node)) {
            return "c";
        } else if (std::holds_alternative<document::String>(node)) {
            return detailed ? "s" : "v";
        } else if (std::holds_alternative<document::Boolean>(node)) {
            return detailed ? "b" : "v";
        } else if (std::holds_alternative<document::Int>(node)) {
            return detailed ? "i" : "v";
        } else if (std::holds_alternative<document::Real>(node)) {
            return detailed ? "r" : "v";
        } else if (std::holds_alternative<document::Null>(node)) {
            return detailed ? "n" : "v";
        } else if (std::holds_alternative<document::Bytes>(node)) {
            return "B";
        } else if (std::holds_alternative<document::Mapping>(node)) {
            absl::StrAppend(&buf, "{");
            for (const auto& v : std::get<document::Mapping>(node).value) {
                absl::StrAppend(&buf, v->structure(detailed));
            }
            absl::StrAppend(&buf, "}");
            return buf;
        } else if (std::holds_alternative<document::Sequence>(node)) {
            absl::StrAppend(&buf, "[");
            for (const auto& v : std::get<document::Sequence>(node).value) {
                absl::StrAppend(&buf, v->structure(detailed));
            }
            absl::StrAppend(&buf, "]");
            return buf;
        } else if (std::holds_alternative<document::Compact>(node)) {
            absl::StrAppend(&buf, "<");
            for (const auto& v : std::get<document::Compact>(node).value) {
                absl::StrAppend(&buf, v->structure(detailed));
            }
            absl::StrAppend(&buf, ">");
            return buf;
        } else if (std::holds_alternative<document::Fragment>(node)) {
            absl::StrAppend(&buf, "(");
            for (const auto& v : std::get<document::Fragment>(node).value) {
                absl::StrAppend(&buf, v->structure(detailed));
            }
            absl::StrAppend(&buf, ")");
            return buf;
        } else {
            LOG(FATAL) << "Invalid document state";
            return "";
        }
    }

    bool has_value() const {
        if (std::holds_alternative<document::Comment>(node)) {
            return false;
        } else if (std::holds_alternative<document::String>(node)) {
            return true;
        } else if (std::holds_alternative<document::Boolean>(node)) {
            return true;
        } else if (std::holds_alternative<document::Int>(node)) {
            return true;
        } else if (std::holds_alternative<document::Real>(node)) {
            return true;
        } else if (std::holds_alternative<document::Null>(node)) {
            return true;
        } else if (std::holds_alternative<document::Bytes>(node)) {
            return true;
        } else if (std::holds_alternative<document::Mapping>(node)) {
            return true;
        } else if (std::holds_alternative<document::Sequence>(node)) {
            return true;
        } else if (std::holds_alternative<document::Compact>(node)) {
            for (const auto& n : std::get<document::Compact>(node).value) {
                if (n->has_value()) return true;
            }
            return false;
        } else if (std::holds_alternative<document::Fragment>(node)) {
            for (const auto& n : std::get<document::Fragment>(node).value) {
                if (n->has_value()) return true;
            }
            return false;
        } else {
            LOG(FATAL) << "Invalid document state";
            return false;
        }
    }

    Location location() const {
        if (std::holds_alternative<document::Comment>(node)) {
            return std::get<document::Comment>(node).location;
        } else if (std::holds_alternative<document::String>(node)) {
            return std::get<document::String>(node).location;
        } else if (std::holds_alternative<document::Boolean>(node)) {
            return std::get<document::Boolean>(node).location;
        } else if (std::holds_alternative<document::Int>(node)) {
            return std::get<document::Int>(node).location;
        } else if (std::holds_alternative<document::Real>(node)) {
            return std::get<document::Real>(node).location;
        } else if (std::holds_alternative<document::Null>(node)) {
            return std::get<document::Null>(node).location;
        } else if (std::holds_alternative<document::Bytes>(node)) {
            return std::get<document::Bytes>(node).location;
        } else if (std::holds_alternative<document::Mapping>(node)) {
            return std::get<document::Mapping>(node).location;
        } else if (std::holds_alternative<document::Sequence>(node)) {
            return std::get<document::Sequence>(node).location;
        } else if (std::holds_alternative<document::Compact>(node)) {
            return std::get<document::Compact>(node).location;
        } else if (std::holds_alternative<document::Fragment>(node)) {
            return std::get<document::Fragment>(node).location;
        } else {
            LOG(FATAL) << "Invalid document state";
            return Location{};
        }
    }

    std::string debug_string() {
        if (std::holds_alternative<document::Comment>(node)) {
            return std::get<document::Comment>(node).debug_string();
        } else if (std::holds_alternative<document::String>(node)) {
            return std::get<document::String>(node).debug_string();
        } else if (std::holds_alternative<document::Boolean>(node)) {
            return std::get<document::Boolean>(node).debug_string();
        } else if (std::holds_alternative<document::Int>(node)) {
            return std::get<document::Int>(node).debug_string();
        } else if (std::holds_alternative<document::Real>(node)) {
            return std::get<document::Real>(node).debug_string();
        } else if (std::holds_alternative<document::Null>(node)) {
            return std::get<document::Null>(node).debug_string();
        } else if (std::holds_alternative<document::Bytes>(node)) {
            return std::get<document::Bytes>(node).debug_string();
        } else if (std::holds_alternative<document::Mapping>(node)) {
            return std::get<document::Mapping>(node).debug_string();
        } else if (std::holds_alternative<document::Sequence>(node)) {
            return std::get<document::Sequence>(node).debug_string();
        } else if (std::holds_alternative<document::Compact>(node)) {
            return std::get<document::Compact>(node).debug_string();
        } else if (std::holds_alternative<document::Fragment>(node)) {
            return std::get<document::Fragment>(node).debug_string();
        } else {
            LOG(FATAL) << "Invalid document state";
            return "Invalid document state";
        }
    }

    Document(document::Comment&& n) : node(std::move(n)) {}
    Document(document::String&& n) : node(std::move(n)) {}
    Document(document::Boolean&& n) : node(std::move(n)) {}
    Document(document::Int&& n) : node(std::move(n)) {}
    Document(document::Real&& n) : node(std::move(n)) {}
    Document(document::Null&& n) : node(std::move(n)) {}
    Document(document::Bytes&& n) : node(std::move(n)) {}
    Document(document::Mapping&& n) : node(std::move(n)) {}
    Document(document::Sequence&& n) : node(std::move(n)) {}
    Document(document::Compact&& n) : node(std::move(n)) {}
    Document(document::Fragment&& n) : node(std::move(n)) {}

    absl::Status append(std::shared_ptr<Document> item) {
        if (std::holds_alternative<document::Mapping>(node)) {
            std::get<document::Mapping>(node).value.push_back(std::move(item));
            return absl::OkStatus();
        } else if (std::holds_alternative<document::Sequence>(node)) {
            std::get<document::Sequence>(node).value.push_back(std::move(item));
            return absl::OkStatus();
        } else if (std::holds_alternative<document::Compact>(node)) {
            std::get<document::Compact>(node).value.push_back(std::move(item));
            return absl::OkStatus();
        } else if (std::holds_alternative<document::Fragment>(node)) {
            std::get<document::Fragment>(node).value.push_back(std::move(item));
            return absl::OkStatus();
        }
        return absl::InvalidArgumentError(
            absl::StrCat("Cannot append to a document node ", type()));
    }

    absl::Status extend(std::shared_ptr<Document> item) {
        if (std::holds_alternative<document::Fragment>(node)) {
            if (std::holds_alternative<document::Fragment>(item->node)) {
                auto& items = std::get<document::Fragment>(item->node).value;
                auto& self = std::get<document::Fragment>(node).value;
                for (auto& i : items) {
                    self.push_back(std::move(i));
                }
            } else {
                std::get<document::Fragment>(node).value.push_back(
                    std::move(item));
            }
            return absl::OkStatus();
        }
        return absl::InvalidArgumentError(
            absl::StrCat("Cannot append to a document node ", type()));
    }

    static std::shared_ptr<Document> Comment(
        std::string value, CommentFormat format = CommentFormat::None,
        Location location = Location{}) {
        return std::make_shared<Document>(
            document::Comment{value, format, location});
    }

    static std::shared_ptr<Document> String(
        std::string value, StringFormat format = StringFormat::None,
        Location location = Location{}) {
        return std::make_shared<Document>(
            document::String{value, format, location});
    }

    static std::shared_ptr<Document> Boolean(bool value,
                                             Location location = Location{}) {
        return std::make_shared<Document>(document::Boolean{value, location});
    }

    static std::shared_ptr<Document> Int(uint64_t value, bool negative,
                                         Base base = Base::Decimal,
                                         uint8_t size = 0,
                                         Location location = Location{}) {
        return std::make_shared<Document>(
            document::Int{value, negative, base, size, location});
    }

    static std::shared_ptr<Document> Int(uint64_t value,
                                         Base base = Base::Decimal,
                                         uint8_t size = 0,
                                         Location location = Location{}) {
        return std::make_shared<Document>(
            document::Int{value, false, base, size, location});
    }

    static std::shared_ptr<Document> Int(int64_t value,
                                         Base base = Base::Decimal,
                                         uint8_t size = 0,
                                         Location location = Location{}) {
        bool neg = value < 0;
        uint64_t v = neg ? -value : value;
        return std::make_shared<Document>(
            document::Int{v, neg, base, size, location});
    }

    static std::shared_ptr<Document> Real(double value,
                                          Location location = Location{}) {
        return std::make_shared<Document>(document::Real{value, location});
    }

    static std::shared_ptr<Document> Null(Location location = Location{}) {
        return std::make_shared<Document>(document::Null{location});
    }

    static std::shared_ptr<Document> Bytes(std::vector<uint8_t> value,
                                           Location location = Location{}) {
        return std::make_shared<Document>(document::Bytes{value, location});
    }

    static std::shared_ptr<Document> Mapping(
        std::vector<std::shared_ptr<Document>> value,
        Location location = Location{}) {
        return std::make_shared<Document>(
            document::Mapping{std::move(value), location});
    }

    static std::shared_ptr<Document> Sequence(
        std::vector<std::shared_ptr<Document>> value,
        Location location = Location{}) {
        return std::make_shared<Document>(
            document::Sequence{std::move(value), location});
    }

    static std::shared_ptr<Document> Compact(std::shared_ptr<Document> v0,
                                             Location location = Location{}) {
        std::vector<std::shared_ptr<Document>> v;
        v.push_back(std::move(v0));
        return std::make_shared<Document>(
            document::Compact{std::move(v), location});
    }

    static std::shared_ptr<Document> Fragment(
        std::shared_ptr<Document> v0 = nullptr,
        std::shared_ptr<Document> v1 = nullptr,
        std::shared_ptr<Document> v2 = nullptr,
        std::shared_ptr<Document> v3 = nullptr,
        Location location = Location{}) {
        std::vector<std::shared_ptr<Document>> v;
        if (v0) v.push_back(std::move(v0));
        if (v1) v.push_back(std::move(v1));
        if (v2) v.push_back(std::move(v2));
        if (v3) v.push_back(std::move(v2));
        return std::make_shared<Document>(
            document::Fragment{std::move(v), location});
    }
    static std::shared_ptr<Document> Fragment(
        std::vector<std::shared_ptr<Document>> value,
        Location location = Location{}) {
        return std::make_shared<Document>(
            document::Fragment{std::move(value), location});
    }
};

static std::string _debug_string(Document* d) { return d->debug_string(); }

}  // namespace ajson

#endif  // EMPTY_PROJECT_AJSON_DOCUMENT_H
