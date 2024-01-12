#ifndef EMPTY_PROJECT_AJSON_TYPES_H
#define EMPTY_PROJECT_AJSON_TYPES_H
#include <cstdint>
#include <map>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"

namespace types {

enum class TypeHint {
    Void,
    Primitive,
    Struct,
    Vector,
    Map,
};

template <typename T>
struct Typename {
    static std::string_view name();
    static absl::StatusOr<T> from_str(absl::string_view str);
    static absl::StatusOr<std::string> to_str(const T& val);
};

template <typename T>
struct Typename<std::vector<T>> {
    static std::string_view name() {
        static std::string name_;
        if (name_.empty()) {
            name_ = absl::StrCat("std::vector<", Typename<T>::name(), ">");
        }
        return name_;
    }
    static absl::StatusOr<T> from_str(absl::string_view str) {
        return absl::UnimplementedError(
            "Cannot convert a vector from a string");
    }
    static absl::StatusOr<std::string> to_str(const T& val) {
        return absl::UnimplementedError("Cannot convert a vector to a string");
    }
};

template <typename K, typename V>
struct Typename<std::map<K, V>> {
    static std::string_view name() {
        static std::string name_;
        if (name_.empty()) {
            name_ = absl::StrCat("std::map<", Typename<K>::name(), ", ",
                                 Typename<V>::name(), ">");
        }
        return name_;
    }
    static absl::StatusOr<std::map<K, V>> from_str(absl::string_view str) {
        return absl::UnimplementedError("Cannot convert a map from a string");
    }
    static absl::StatusOr<std::string> to_str(const std::map<K, V>& val) {
        return absl::UnimplementedError("Cannot convert a map to a string");
    }
};

struct Type {
    template <typename T>
    static std::string_view name() {
        return Typename<T>::name();
    }

    template <typename T>
    static std::string_view of_val(const T& value) {
        return Typename<T>::name();
    }

    template <typename T>
    static absl::StatusOr<T> from_str(absl::string_view str) {
        return Typename<T>::from_str(str);
    }

    template <typename T>
    static absl::StatusOr<std::string> to_str(const T& val) {
        return Typename<T>::to_str(val);
    }
};

#define DEFINE_TYPE(ty)                                             \
    namespace types {                                               \
    template <>                                                     \
    struct Typename<ty> {                                           \
        static std::string_view name() { return #ty; }              \
        static absl::StatusOr<ty> from_str(absl::string_view str) { \
            return absl::UnimplementedError("Cannot convert a " #ty \
                                            " from a string");      \
        }                                                           \
        static absl::StatusOr<std::string> to_str(const ty& val) {  \
            return absl::UnimplementedError("Cannot convert a " #ty \
                                            " to a string");        \
        }                                                           \
    };                                                              \
    }                                                               \
    extern "C" const int ___never_referenced_here_to_eat_a_semicolon[]

#define DEFINE_TYPE_CONVERSIONS(ty, from_, to_)        \
    namespace types {                                  \
    template <>                                        \
    struct Typename<ty> {                              \
        static std::string_view name() { return #ty; } \
        from_ to_                                      \
    };                                                 \
    }                                                  \
    extern "C" const int ___never_referenced_here_to_eat_a_semicolon[]

#define DEFINE_UINT_TYPE(ty)                                           \
    DEFINE_TYPE_CONVERSIONS(                                           \
        ty,                                                            \
        static absl::StatusOr<ty> from_str(absl::string_view str) {    \
            uint64_t val;                                              \
            if (absl::SimpleAtoi(str, &val)) {                         \
                return static_cast<ty>(val);                           \
            } else {                                                   \
                return absl::InvalidArgumentError(                     \
                    absl::StrCat("Failed integer conversion: ", str)); \
            }                                                          \
        },                                                             \
        static absl::StatusOr<std::string> to_str(const ty& val) {     \
            return absl::StrCat(val);                                  \
        })

#define DEFINE_INT_TYPE(ty)                                            \
    DEFINE_TYPE_CONVERSIONS(                                           \
        ty,                                                            \
        static absl::StatusOr<ty> from_str(absl::string_view str) {    \
            int64_t val;                                               \
            if (absl::SimpleAtoi(str, &val)) {                         \
                return static_cast<ty>(val);                           \
            } else {                                                   \
                return absl::InvalidArgumentError(                     \
                    absl::StrCat("Failed integer conversion: ", str)); \
            }                                                          \
        },                                                             \
        static absl::StatusOr<std::string> to_str(const ty& val) {     \
            return absl::StrCat(val);                                  \
        })

}  // namespace types

DEFINE_TYPE_CONVERSIONS(
    void,
    static absl::StatusOr<bool> from_str(absl::string_view str) {
        return absl::UnimplementedError("Cannot convert a void from a string");
    },
    static absl::StatusOr<std::string> to_str(void) {
        return absl::UnimplementedError("Cannot convert a void to a string");
    });

DEFINE_TYPE_CONVERSIONS(
    bool,
    static absl::StatusOr<bool> from_str(absl::string_view str) {
        if (str == "true") return true;
        if (str == "false") return false;
        return absl::InvalidArgumentError(
            absl::StrCat("Failed boolean conversion: ", str));
    },
    static absl::StatusOr<std::string> to_str(const bool& val) {
        return absl::StrCat(val ? "true" : "false");
    });

DEFINE_UINT_TYPE(uint8_t);
DEFINE_UINT_TYPE(uint16_t);
DEFINE_UINT_TYPE(uint32_t);
DEFINE_UINT_TYPE(uint64_t);
DEFINE_INT_TYPE(int8_t);
DEFINE_INT_TYPE(int16_t);
DEFINE_INT_TYPE(int32_t);
DEFINE_INT_TYPE(int64_t);
DEFINE_TYPE_CONVERSIONS(
    float,
    static absl::StatusOr<float> from_str(absl::string_view str) {
        float val;
        if (absl::SimpleAtof(str, &val)) {
            return val;
        } else {
            return absl::InvalidArgumentError(
                absl::StrCat("Failed float conversion: ", str));
        }
    },
    static absl::StatusOr<std::string> to_str(const float& val) {
        return absl::StrCat(val);
    });
DEFINE_TYPE_CONVERSIONS(
    double,
    static absl::StatusOr<double> from_str(absl::string_view str) {
        double val;
        if (absl::SimpleAtod(str, &val)) {
            return val;
        } else {
            return absl::InvalidArgumentError(
                absl::StrCat("Failed double conversion: ", str));
        }
    },
    static absl::StatusOr<std::string> to_str(const double& val) {
        return absl::StrCat(val);
    });
DEFINE_TYPE_CONVERSIONS(
    std::string,
    static absl::StatusOr<std::string> from_str(absl::string_view str) {
        return absl::StrCat(str);
    },
    static absl::StatusOr<std::string> to_str(const std::string& val) {
        return val;
    });

#endif  // EMPTY_PROJECT_AJSON_TYPES_H
