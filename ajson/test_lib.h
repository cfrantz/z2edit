#ifndef EMPTY_PROJECT_AJSON_TEST_TYPES_H
#define EMPTY_PROJECT_AJSON_TEST_TYPES_H

#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "ajson/reflect.h"

#ifdef PYTHON_TESTING
#define maybe_pybind11 pybind11
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#else
#define maybe_pybind11 none
#endif

struct Foo : public ajson::Reflection {
    Foo() = default;
    Foo(uint32_t a_, bool b_, std::string c_) : a(a_), b(b_), c(c_) {}
    uint32_t a = 170;
    bool b = false;
    std::string c = "hello";
    std::optional<int32_t> d;

    // clang-format off
    OBJECT_CONFIG(
        /*typename=*/ Foo,
        /*features=*/ (reflection, maybe_pybind11),
        /*field_list ... */
        (a, .comment="Field A", .format=ajson::format::Hex),
        b,
        c,
        d
    )
    // clang-format on
};
DEFINE_TYPE(Foo);

constexpr char MARY_HAD_A_LAMB[] =
    R"(Mary had a little lamb;
It's fleece was white as snow.
Everywhere that Mary went
the lamb was sure to go.)";

struct Bar : public ajson::Reflection {
    Foo foo;
    std::vector<Foo> foos = {
        {170, false, "hello"},
        {85, true, "bye"},
    };
    std::vector<int32_t> ints = {-2, -1, 0, 1, 2};
    std::map<std::string, int32_t> map = {
        {"hello", 0},
        {"big", 1},
        {"bad", 2},
        {"world", 3},
    };
    float pi = 3.14159;
    double e = 2.71818;
    std::string poem = MARY_HAD_A_LAMB;

    // clang-format off
    OBJECT_CONFIG(
        /*typename=*/ Bar,
        /*features=*/ (reflection, maybe_pybind11),
        /*field_list ... */
        (foo, .format=ajson::format::Compact),
        foos,
        ints,
        map,
        pi,
        e,
        (poem, .format=ajson::format::Block)
    )
    // clang-format on
};
DEFINE_TYPE(Bar);

struct Baz : public ajson::Reflection {
    std::string name;
    std::variant<int32_t, std::string, Foo> var;

    OBJECT_CONFIG(
        /*typename=*/Baz,
        /*features=*/(reflection, maybe_pybind11),
        /*field_list ... */
        name, (var, .metadata = "variant:ival,sval,foo"))
};
DEFINE_TYPE(Baz);

#endif  // EMPTY_PROJECT_AJSON_TEST_TYPES_H
