#include "ajson/deserialize.h"

#include "ajson/ajson.h"
#include "ajson/relax.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "test_lib.h"

namespace ajson {
namespace {

TEST(Deserialize, Foo) {
    const std::string text = R"({
        "a": 1,
        "b": true,
        "c": "greeting",
        "d": 9999
    })";
    auto doc = Relax().parse(text);
    EXPECT_TRUE(doc.ok());

    Foo foo;
    Deserializer d;
    auto result = d.deserialize(foo._ref(), doc->get());
    EXPECT_TRUE(result.ok());

    EXPECT_EQ(foo.a, 1);
    EXPECT_EQ(foo.b, true);
    EXPECT_EQ(foo.c, "greeting");
    EXPECT_EQ(*foo.d, 9999);
}

TEST(Deserialize, Bar) {
    const std::string text = R"({
        "foo": {
            "a": 1,
            "b": true,
            "c": "greeting"        
        },
        ints: [99, 100],
        foos: [
            { "a": 66 }
        ],
        map: {
            "cruel": 5
        }
    })";
    auto doc = Relax().parse(text);
    EXPECT_TRUE(doc.ok());

    Bar bar;
    Deserializer d;
    auto result = d.deserialize(bar._ref(), doc->get());
    EXPECT_TRUE(result.ok());
    auto str = JsonEncoder().encode(bar);
    LOG(INFO) << *str;
}

}  // namespace
}  // namespace ajson
