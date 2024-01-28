#include "ajson/ajson.h"

#include "ajson/reflect.h"
#include "ajson/serialize.h"
#include "ajson/test_lib.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace ajson {
namespace {

TEST(JsonEncode, StructFoo) {
    Foo foo;
    JsonEncoder enc;
    Serializer ser;

    auto doc = ser.serialize(Ref::New(foo, "foo"));
    EXPECT_TRUE(doc.ok());
    auto val = enc.encode(doc->get());
    EXPECT_TRUE(val.ok());
    std::cout << *val << "\n";
}

TEST(JsonEncode, StructBar) {
    Bar bar;
    JsonEncoder enc = JsonEncoder::Hjson();
    Serializer ser;

    bar.foo.d = 9999;
    auto doc = ser.serialize(Ref::New(bar, "bar"));
    EXPECT_TRUE(doc.ok());
    auto val = enc.encode(doc->get());
    EXPECT_TRUE(val.ok());
    std::cout << *val << "\n";
}

TEST(JsonEncode, StructBaz) {
    Baz baz;
    JsonEncoder enc;
    Serializer ser;

    baz.var = 9999;
    LOG(INFO) << "serialize";
    auto doc = ser.serialize(Ref::New(baz, "baz"));
    EXPECT_TRUE(doc.ok());
    LOG(INFO) << "encode";
    auto val = enc.encode(doc->get());
    EXPECT_TRUE(val.ok());
    std::cout << *val << "\n";
}

}  // namespace
}  // namespace ajson
