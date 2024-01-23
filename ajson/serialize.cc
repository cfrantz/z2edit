#include "ajson/serialize.h"

#include "ajson/document.h"
#include "ajson/reflect.h"
#include "util/status.h"

namespace ajson {

Serializer::State Serializer::apply_annotation(const Annotation& a) {
    State old = state_;
    if (a.format & format::Compact) {
        state_.compact = true;
    }
    uint32_t base = a.format & 0x000000FE;
    if (base) state_.base = static_cast<Base>(base);
    uint32_t str = (a.format & 0x0000F000) >> 12;
    if (str) {
        state_.str = static_cast<StringFormat>(str);
    }
    return old;
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::primitive(Ref r) {
    ASSIGN_OR_RETURN(auto type, r.type());
    if (type == "bool") {
        return Document::Boolean(*r.value<bool>());
    } else if (type == "uint8_t") {
        return Document::Int(uint64_t(*r.value<uint8_t>()), state_.base);
    } else if (type == "uint16_t") {
        return Document::Int(uint64_t(*r.value<uint16_t>()), state_.base);
    } else if (type == "uint32_t") {
        return Document::Int(uint64_t(*r.value<uint32_t>()), state_.base);
    } else if (type == "uint64_t") {
        return Document::Int(uint64_t(*r.value<uint64_t>()), state_.base);
    } else if (type == "int8_t") {
        return Document::Int(int64_t(*r.value<int8_t>()), state_.base);
    } else if (type == "int16_t") {
        return Document::Int(int64_t(*r.value<int16_t>()), state_.base);
    } else if (type == "int32_t") {
        return Document::Int(int64_t(*r.value<int32_t>()), state_.base);
    } else if (type == "int64_t") {
        return Document::Int(int64_t(*r.value<int64_t>()), state_.base);
    } else if (type == "float") {
        return Document::Real(*r.value<float>());
    } else if (type == "double") {
        return Document::Real(*r.value<double>());
    } else if (type == "std::string") {
        return Document::String(*r.value<std::string>(), state_.str);
    } else {
        return absl::InvalidArgumentError(
            absl::StrCat(type, " is not a primitive type"));
    }
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::structure(Ref r) {
    std::vector<std::shared_ptr<Document>> m;
    ASSIGN_OR_RETURN(auto fields, r.fields());
    for (const auto& [key, annotation] : fields) {
        auto state = apply_annotation(annotation);
        std::shared_ptr<Document> comment;
        if (!annotation.comment.empty()) {
            comment = Document::Comment(std::string(annotation.comment));
        }
        ASSIGN_OR_RETURN(auto item, r.getitem(key));
        ASSIGN_OR_RETURN(auto value, serialize(item));
        auto frag = Document::Fragment(std::move(comment),
                                       Document::String(key), std::move(value));
        m.emplace_back(std::move(frag));
        state_ = state;
    }
    return Document::Mapping(std::move(m));
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::vector(Ref r) {
    document::Sequence s;
    ASSIGN_OR_RETURN(size_t len, r.size());
    for (size_t i = 0; i < len; ++i) {
        ASSIGN_OR_RETURN(auto item, r.getitem(i));
        ASSIGN_OR_RETURN(auto value, serialize(item));
        s.value.emplace_back(std::move(value));
    }
    return std::make_shared<Document>(std::move(s));
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::optional(Ref r) {
    document::Sequence s;
    ASSIGN_OR_RETURN(size_t has_value, r.size());
    if (has_value) {
        ASSIGN_OR_RETURN(auto item, r.getitem("value"));
        return serialize(item);
    } else {
        return Document::Null();
    }
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::map(Ref r) {
    std::vector<std::shared_ptr<Document>> m;
    ASSIGN_OR_RETURN(auto fields, r.fields());
    // TODO: deal with non-string map keys.
    for (const auto& [key, annotation] : fields) {
        auto state = apply_annotation(annotation);
        ASSIGN_OR_RETURN(auto item, r.getitem(key));
        ASSIGN_OR_RETURN(auto value, serialize(item));
        auto frag = Document::Fragment(Document::String(key), std::move(value));
        m.emplace_back(std::move(frag));
        state_ = state;
    }
    return Document::Mapping(std::move(m));
}

absl::StatusOr<std::shared_ptr<Document>> Serializer::serialize(Ref r) {
    ASSIGN_OR_RETURN(auto hint, r.hint());
    std::shared_ptr<Document> doc;
    switch (hint) {
        case ::types::TypeHint::Primitive: {
            ASSIGN_OR_RETURN(doc, primitive(r));
            break;
        }
        case ::types::TypeHint::Struct: {
            ASSIGN_OR_RETURN(doc, structure(r));
            break;
        }
        case ::types::TypeHint::Vector: {
            ASSIGN_OR_RETURN(doc, vector(r));
            break;
        }
        case ::types::TypeHint::Optional: {
            ASSIGN_OR_RETURN(doc, optional(r));
            break;
        }
        case ::types::TypeHint::Map: {
            ASSIGN_OR_RETURN(doc, map(r));
            break;
        }
        default:
            return absl::UnimplementedError("Not implemented");
    }
    if (state_.compact) {
        doc = Document::Compact(std::move(doc));
    }
    return doc;
}

}  // namespace ajson
