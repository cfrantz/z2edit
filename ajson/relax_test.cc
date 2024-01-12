#include "ajson/relax.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace ajson {
namespace {
Relax::Options pure = {
    .comma_trailing = false,
    .comma_optional = false,
    .number_bin = false,
    .number_hex = false,
    .number_oct = false,
    .number_plus = false,
    .number_lax_decimal = false,
    .string_single_quote = false,
    .string_unquoted = false,
    .string_ident = false,
    .string_json5_multiline = false,
    .string_hjson_multiline = false,
    .comment_slash = false,
    .comment_hash = false,
    .comment_block = false,
};

TEST(RelaxParse, QuotedString) {
    const std::string value = R"("foo")";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "foo");
}

TEST(RelaxParse, SingleQuotedString) {
    const std::string value = R"('foo')";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_EQ(strict.status().message(), "single quote not allowed at 1:1");

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "foo");
}

TEST(RelaxParse, BareString) {
    const std::string value = "the quick brown fox\n";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_EQ(strict.status().message(), "unquoted strings not allowed at 1:1");

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "the quick brown fox");
}

TEST(RelaxParse, StringEscapes) {
    const std::string value = R"("\b\t\n\f\r\"\'\\\/\u20ac")";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "\b\t\n\f\r\"'\\/\xE2\x82\xAC");
}

TEST(RelaxParse, TripleQuotedString) {
    const std::string value = R"("""
foo
bar
baz
""")";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_EQ(strict.status().message(),
              "triple quoted string not allowed at 1:1");

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "\nfoo\nbar\nbaz\n");
}

TEST(RelaxParse, Json5MultilineString) {
    const std::string value = R"("The quick brown fox\
jumped over the lazy dog")";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_EQ(strict.status().message(),
              "json5 multiline strings not allowed at 2:1");

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::String);
    auto& string = doc->as<document::String>();
    EXPECT_TRUE(string.ok());
    EXPECT_EQ(string->value, "The quick brown fox\njumped over the lazy dog");
}

TEST(RelaxParse, BooleanTrue) {
    const std::string value = "true";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Boolean);
    auto& b = doc->as<document::Boolean>();
    EXPECT_TRUE(b.ok());
    EXPECT_EQ(b->value, true);
}

TEST(RelaxParse, BooleanFalse) {
    const std::string value = "false";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Boolean);
    auto& b = doc->as<document::Boolean>();
    EXPECT_TRUE(b.ok());
    EXPECT_EQ(b->value, false);
}

TEST(RelaxParse, Null) {
    const std::string value = "null";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Null);
    auto& n = doc->as<document::Null>();
    EXPECT_TRUE(n.ok());
}

TEST(RelaxParse, Integer) {
    struct Cases {
        std::string input;
        int64_t value;
        bool ok;
        std::string message;
    };
    Cases tests[] = {
        {"0", 0, true, ""},
        {"12345", 12345, true, ""},
        {"-12345", -12345, true, ""},
        {"+12345", 12345, false, "'+' not allowed at 1:1"},
        {"0x1234", 0x1234, false, "hex literals not allowed at 1:1"},
        {"0b1101", 0xD, false, "binary literals not allowed at 1:1"},
        {"0o1717", 01717, false, "octal literals not allowed at 1:1"},
        {"0377", 0377, false, "octal literals not allowed at 1:1"},
    };
    for (const auto& t : tests) {
        auto doc_or = Relax().parse(t.input);
        EXPECT_TRUE(doc_or.ok());
        auto strict = Relax(pure).parse(t.input);
        EXPECT_EQ(strict.ok(), t.ok);
        EXPECT_EQ(strict.status().message(), t.message);

        auto& doc = *doc_or;
        EXPECT_EQ(doc->type(), document::Type::Int);
        auto& i = doc->as<document::Int>();
        EXPECT_TRUE(i.ok());
        EXPECT_EQ(i->as<int64_t>(), t.value);
    }
}

TEST(RelaxParse, Real) {
    struct Cases {
        std::string input;
        double value;
        bool ok;
        std::string message;
    };
    Cases tests[] = {
        {"0.0", 0, true, ""},
        {"3.14", 3.14, true, ""},
        {"-3.14", -3.14, true, ""},
        {".314", 0.314, false, "'.' not allowed at 1:1"},
        {"-.314", -0.314, false, "'.' not allowed at 1:2"},
    };
    for (const auto& t : tests) {
        auto doc_or = Relax().parse(t.input);
        EXPECT_TRUE(doc_or.ok());
        auto strict = Relax(pure).parse(t.input);
        EXPECT_EQ(strict.ok(), t.ok);
        EXPECT_EQ(strict.status().message(), t.message);

        auto& doc = *doc_or;
        EXPECT_EQ(doc->type(), document::Type::Real);
        auto& r = doc->as<document::Real>();
        EXPECT_TRUE(r.ok());
        EXPECT_EQ(r->value, t.value);
    }
}

TEST(RelaxParse, Sequence) {
    const std::string value = "[1, 2, 3, 4, 5]";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Sequence);
    EXPECT_EQ(doc->structure(true), "[iiiii]");
}

TEST(RelaxParse, EmptySequence) {
    const std::string value = "[ ]";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Sequence);
    EXPECT_EQ(doc->structure(true), "[]");
}

TEST(RelaxParse, CommentEmptySequence) {
    const std::string value = R"([
# Foo
])";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_FALSE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Sequence);
    EXPECT_EQ(doc->structure(true), "[c]");
}

TEST(RelaxParse, CommentSequence) {
    const std::string value = R"([
# First
1
# Second
2
3 #Third
4, # Fourth
])";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_FALSE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Sequence);
    EXPECT_EQ(doc->structure(true), "[(ci)(ci)(ic)(ic)]");
}

TEST(RelaxParse, Mapping) {
    const std::string value = R"({
        "a": 1,
        "b": 2,
        "c": 3
})";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Mapping);
    EXPECT_EQ(doc->structure(true), "{(si)(si)(si)}");
}

TEST(RelaxParse, EmptyMapping) {
    const std::string value = "{ }";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_TRUE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Mapping);
    EXPECT_EQ(doc->structure(true), "{}");
}

TEST(RelaxParse, CommentEmptyMapping) {
    const std::string value = R"({
# Foo
})";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_FALSE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Mapping);
    EXPECT_EQ(doc->structure(true), "{c}");
}

TEST(RelaxParse, CommentMapping) {
    const std::string value = R"({
        # Comment A
        "a": 1
        # Comment B
        "b": 2 # Comment B2
        "c": 3, # Comment C
})";
    auto doc_or = Relax().parse(value);
    EXPECT_TRUE(doc_or.ok());
    auto strict = Relax(pure).parse(value);
    EXPECT_FALSE(strict.ok());

    auto& doc = *doc_or;
    EXPECT_EQ(doc->type(), document::Type::Mapping);
    EXPECT_EQ(doc->structure(true), "{(csi)(csic)(sic)}");
}

}  // namespace
}  // namespace ajson
