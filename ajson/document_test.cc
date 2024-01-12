#include "ajson/document.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace ajson {
namespace {

TEST(Document, String) {
    auto doc = Document::String("foo");
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "foo");
}

}  // namespace
}  // namespace ajson
