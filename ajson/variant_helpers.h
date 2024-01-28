#ifndef EMPTY_PROJECT_AJSON_VARIANT_HELPERS_H
#define EMPTY_PROJECT_AJSON_VARIANT_HELPERS_H

#include <cstdint>
#include <limits>

#include "util/status.h"

namespace internal {

template <size_t I>
struct VariantHelper {
    template <typename... T>
    static absl::Status emplace(size_t i, std::variant<T...>& v) {
        if (i == I - 1) {
            if (i != v.index()) {
                v.template emplace<I - 1>();
            }
            return absl::OkStatus();
        } else {
            return VariantHelper<I - 1>::emplace(i, v);
        }
    }

    template <typename... T>
    static absl::StatusOr<Ref> get(size_t i, std::string_view key,
                                   std::variant<T...>& v) {
        if (i == I - 1) {
            size_t index = v.index();
            if (i == index) {
                return Ref::New(std::get<I - 1>(v), key);
            } else {
                return absl::InvalidArgumentError(
                    absl::StrCat("expected index ", i, ", but got ", index));
            }
        } else {
            return VariantHelper<I - 1>::get(i, key, v);
        }
    }
};

template <>
struct VariantHelper<0> {
    template <typename... T>
    static absl::Status emplace(size_t i, std::variant<T...>& v) {
        return absl::NotFoundError(absl::StrCat("variant index ", i));
    }
    template <typename... T>
    static absl::StatusOr<Ref> get(size_t i, std::string_view key,
                                   std::variant<T...>& v) {
        return absl::NotFoundError(absl::StrCat("variant index ", i));
    }
};

}  // namespace internal
#endif  // EMPTY_PROJECT_AJSON_VARIANT_HELPERS_H
