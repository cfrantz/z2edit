#include "ajson/reflect.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "test_lib.h"

namespace ajson {
namespace {

using ::testing::ElementsAre;
using ::testing::Key;

// Tests that type-erased Ref works with primitive types.
TEST(RefPrimitiveTest, Bool) {
    bool value = true;
    Ref b = Ref::New(value, "value");
    EXPECT_EQ(*b.value<bool>(), true);

    *b.value<bool>() = false;
    EXPECT_EQ(*b.value<bool>(), false);

    EXPECT_FALSE(b.getitem("x").ok());
}

// Tests that type-erased Ref works with structures.
TEST(RefStructureTest, StructFoo) {
    Foo foo;
    Ref f = Ref::New(foo, "foo");
    EXPECT_EQ(*f.getitem("a")->value<uint32_t>(), 170);
}

// Tests that type-erased Ref works with nested structures.
TEST(RefStructureTest, StructBar) {
    Bar bar;
    Foo f2 = {10, true, "bye"};
    Ref b = Ref::New(bar, "bar");
    // Get an item from the Ref object.
    EXPECT_EQ(b.getitem("foo")->value<Foo>()->a, 170);
    // Get a path from the Ref object.
    EXPECT_EQ(*b.get("/foo/a")->value<uint32_t>(), 170);
    // Get a path from the struct itself.
    EXPECT_EQ(*bar._get("/foo/a")->value<uint32_t>(), 170);
    // Assign an item via the Ref object.
    *b.getitem("foo")->value<Foo>() = f2;
    EXPECT_EQ(bar.foo.a, 10);
}

// Tests that type-erased Ref works with vectors.
TEST(RefVectorTest, Primitive) {
    std::vector<int16_t> values = {0, 1, 2, 3, 4, 5};
    Ref v = Ref::New(values, "values");
    EXPECT_EQ(*v.getitem("0")->value<int16_t>(), 0);
    EXPECT_EQ(*v.getitem("4")->value<int16_t>(), 4);
    EXPECT_FALSE(v.getitem("offset").ok());
}

// Tests that type-erased Ref works with optional.
TEST(RefOptionalTest, Primitive) {
    std::optional<int16_t> opt;
    Ref v = Ref::New(opt, "opt");
    EXPECT_EQ(*v.size(), 0);
    EXPECT_FALSE(v.getitem("value").ok());

    EXPECT_TRUE(v.add("value").ok());
    *v.getitem("value")->value<int16_t>() = 55;
    EXPECT_EQ(*v.getitem("value")->value<int16_t>(), 55);

    EXPECT_TRUE(v.add("null").ok());
    EXPECT_FALSE(v.getitem("value").ok());
}

// Tests that type-erased Ref works with maps.
TEST(RefMapTest, String) {
    std::map<std::string, std::string> values = {
        {"a", "one"},
        {"b", "two"},
        {"c", "three"},
    };
    Ref v = Ref::New(values, "values");
    auto fields = *v.fields();
    EXPECT_THAT(fields, ElementsAre(Key("a"), Key("b"), Key("c")));

    EXPECT_EQ(*v.getitem("a")->value<std::string>(), "one");
    EXPECT_EQ(*v.getitem("b")->value<std::string>(), "two");
    EXPECT_EQ(*v.getitem("c")->value<std::string>(), "three");
    EXPECT_FALSE(v.getitem("d").ok());
}

// Tests that type-erased Ref works with maps.
TEST(RefMapTest, Integer) {
    std::map<uint32_t, std::string> values = {
        {1, "one"},
        {2, "two"},
        {3, "three"},
    };
    Ref v = Ref::New(values, "values");
    auto fields = *v.fields();
    EXPECT_THAT(fields, ElementsAre(Key("1"), Key("2"), Key("3")));

    EXPECT_EQ(*v.getitem("1")->value<std::string>(), "one");
    EXPECT_EQ(*v.getitem("2")->value<std::string>(), "two");
    EXPECT_EQ(*v.getitem("3")->value<std::string>(), "three");
    EXPECT_FALSE(v.getitem("4").ok());
}

// Tests Reflection on a structure.
TEST(Reflection, StructFoo) {
    Foo f;

    auto fields = f._fields();
    EXPECT_THAT(fields, ElementsAre(Key("a"), Key("b"), Key("c")));

    auto z = f._get("z");
    EXPECT_FALSE(z.ok());

    absl::StatusOr<Ref> a = f._get("a");
    EXPECT_EQ(*a->value<uint32_t>(), 170);
    absl::StatusOr<Ref> b = f._get("b");
    EXPECT_EQ(*b->value<bool>(), false);
    absl::StatusOr<Ref> c = f._get("c");
    EXPECT_EQ(*c->value<std::string>(), "hello");
}

}  // namespace
}  // namespace ajson
