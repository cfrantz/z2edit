#ifndef EMPTY_PROJECT_AJSON_REFLECT_MACROS_H
#define EMPTY_PROJECT_AJSON_REFLECT_MACROS_H

#include "util/macros.h"
// clang-format off
#define __field_map2(fld, ...) { \
        #fld, \
        ::ajson::Annotation{ \
            .type = ::types::Type::of_val(fld), \
            __VA_ARGS__ \
        } \
    },
#define __field_map(fld, ...) \
    IIF(NOT(PP_NARG(__VA_ARGS__)))( \
        __field_map2(fld, /*nothing*/) \
        , \
        __field_map2(fld, __VA_ARGS__) \
    )
#define _field_map(x) \
    IIF(IS_PAREN(x))(__field_map2 x, __field_map2(x, /*nothing*/))

#define __get_field(fld, ...) \
    if (key == #fld) return ::ajson::Ref::New(fld, #fld, ::ajson::Annotation{__VA_ARGS__}.metadata);
#define _get_field(x) \
    IIF(IS_PAREN(x))(__get_field x, __get_field(x))

#define OBJECT_FEATURE_none(typename_, ...) /* empty featue */

#define OBJECT_FEATURE_reflection(typename_, ...) \
    ::ajson::Ref _ref() override { \
        return ::ajson::Ref::New(*this, #typename_); \
    } \
    std::map<std::string, ::ajson::Annotation> _fields() override { \
        static std::map<std::string, ::ajson::Annotation> fields { \
            APPLYX(_field_map, ##__VA_ARGS__) \
        }; \
        return fields; \
    } \
    absl::StatusOr<::ajson::Ref> _getitem(std::string_view key) override { \
        APPLYX(_get_field, ##__VA_ARGS__) \
        return absl::NotFoundError("No such field"); \
    }

#define __pybind11_registration(field_, ...) \
    .def_readwrite(#field_, &Self::field_)
#define _pybind11_registration(x) \
    IIF(IS_PAREN(x))(__pybind11_registration x, __pybind11_registration(x))

#define OBJECT_FEATURE_pybind11(typename_, ...) \
    static ::pybind11::class_<typename_> \
    pybind11_bind(::pybind11::module_& pybind11_module_) { \
        using Self = typename_; \
        return ::pybind11::class_<typename_>(pybind11_module_, #typename_) \
            .def(::pybind11::init<>()) \
            APPLYX(_pybind11_registration, __VA_ARGS__) \
            ; \
    }

#define __feature_name(...) \
    APPLYW(PASTE_LIST, OBJECT_FEATURE_, ##__VA_ARGS__) OBJECT_FEATURE_none
#define _feature_name(x) \
    IIF(IS_PAREN(x))(__feature_name x, __feature_name(x))
#define _apply_features(features_, ...) \
    APPLYW(CALL, (__VA_ARGS__), _feature_name(features_))

/**
 * Object Configuration by feature:
 *
 * OBJECT_CONFIG()
 */
#define OBJECT_CONFIG(typename_, features_, ...) \
    _apply_features(features_, typename_, ##__VA_ARGS__)

// clang-format on
#endif  // EMPTY_PROJECT_AJSON_REFLECT_MACROS_H
