#include "ajson/deserialize.h"

#include "absl/log/log.h"
#include "ajson/document.h"
#include "ajson/reflect.h"

namespace ajson {
absl::Status error(std::string_view message, const Location& loc) {
    return absl::InvalidArgumentError(
        absl::StrCat(message, " at ", loc.location()));
}

absl::Status Deserializer::primitive(Ref r, const Document* doc) {
    ASSIGN_OR_RETURN(auto type, r.type());
    if (type == "bool") {
        if (doc->type() != document::Type::Boolean) {
            return error(
                absl::StrCat("expecting boolean but found ", doc->type_name()),
                doc->location());
        }
        *r.value<bool>() = doc->as<document::Boolean>()->value;
    } else if (type == "uint8_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<uint8_t>() = doc->as<document::Int>()->as<uint8_t>();
    } else if (type == "uint16_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<uint16_t>() = doc->as<document::Int>()->as<uint16_t>();
    } else if (type == "uint32_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<uint32_t>() = doc->as<document::Int>()->as<uint32_t>();
    } else if (type == "uint64_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<uint64_t>() = doc->as<document::Int>()->as<uint64_t>();
    } else if (type == "int8_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<int8_t>() = doc->as<document::Int>()->as<int8_t>();
    } else if (type == "int16_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<int16_t>() = doc->as<document::Int>()->as<int16_t>();
    } else if (type == "int32_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<int32_t>() = doc->as<document::Int>()->as<int32_t>();
    } else if (type == "int64_t") {
        if (doc->type() != document::Type::Int) {
            return error(
                absl::StrCat("expecting integer but found ", doc->type_name()),
                doc->location());
        }
        *r.value<int64_t>() = doc->as<document::Int>()->as<int64_t>();
    } else if (type == "float") {
        if (doc->type() != document::Type::Real) {
            return error(
                absl::StrCat("expecting real but found ", doc->type_name()),
                doc->location());
        }
        *r.value<float>() = doc->as<document::Real>()->value;
    } else if (type == "double") {
        if (doc->type() != document::Type::Real) {
            return error(
                absl::StrCat("expecting real but found ", doc->type_name()),
                doc->location());
        }
        *r.value<double>() = doc->as<document::Real>()->value;
    } else if (type == "std::string") {
        if (doc->type() != document::Type::String) {
            return error(
                absl::StrCat("expecting string but found ", doc->type_name()),
                doc->location());
        }
        *r.value<std::string>() = doc->as<document::String>()->value;
    } else {
        return absl::InvalidArgumentError(
            absl::StrCat(type, " is not a primitive type"));
    }
    return absl::OkStatus();
}

absl::Status Deserializer::structure(Ref r, const Document* doc) {
    if (doc->type() != document::Type::Mapping) {
        return error(
            absl::StrCat("expecting mapping but found ", doc->type_name()),
            doc->location());
    }
    for (const auto& kvpair : doc->as<document::Mapping>()->value) {
        if (!kvpair->has_value()) continue;
        if (kvpair->type() != document::Type::Fragment) {
            return error(absl::StrCat("expecting fragment but found ",
                                      kvpair->type_name()),
                         kvpair->location());
        }
        const Document *k = nullptr, *v = nullptr;
        for (const auto& item : kvpair->as<document::Fragment>()->value) {
            if (!item->has_value()) continue;
            if (!k) {
                k = item.get();
            } else if (!v) {
                v = item.get();
            } else {
                return error("too many values in key-value pair",
                             item->location());
            }
        }
        if (!(k && v)) {
            return error("too few values in key-value pair",
                         kvpair->location());
        }
        if (k->type() != document::Type::String) {
            return error(
                absl::StrCat("expecting string but found ", k->type_name()),
                k->location());
        }
        ASSIGN_OR_RETURN(auto item,
                         r.getitem(k->as<document::String>()->value));
        RETURN_IF_ERROR(deserialize(item, v));
    }
    return absl::OkStatus();
}

absl::Status Deserializer::vector(Ref r, const Document* doc) {
    if (doc->type() != document::Type::Sequence) {
        return error(
            absl::StrCat("expecting sequence but found ", doc->type_name()),
            doc->location());
    }
    for (const auto& item : doc->as<document::Sequence>()->value) {
        if (!item->has_value()) continue;
        const Document* value = nullptr;
        if (item->type() == document::Type::Fragment) {
            for (const auto& i : item->as<document::Fragment>()->value) {
                if (!i->has_value()) continue;
                if (!value) {
                    value = i.get();
                } else {
                    return error("too many values in fragment", i->location());
                }
            }
            if (!value) {
                return error("too few values in fragment", item->location());
            }
        } else {
            value = item.get();
        }
        ASSIGN_OR_RETURN(size_t n, r.size());
        RETURN_IF_ERROR(r.add(""));
        ASSIGN_OR_RETURN(auto slot, r.getitem(n));
        RETURN_IF_ERROR(deserialize(slot, value));
    }
    return absl::OkStatus();
}

absl::Status Deserializer::map(Ref r, const Document* doc) {
    if (doc->type() != document::Type::Mapping) {
        return error(
            absl::StrCat("expecting mapping but found ", doc->type_name()),
            doc->location());
    }
    for (const auto& kvpair : doc->as<document::Mapping>()->value) {
        if (!kvpair->has_value()) continue;
        if (kvpair->type() != document::Type::Fragment) {
            return error(absl::StrCat("expecting fragment but found ",
                                      kvpair->type_name()),
                         kvpair->location());
        }
        const Document *k = nullptr, *v = nullptr;
        for (const auto& item : kvpair->as<document::Fragment>()->value) {
            if (!item->has_value()) continue;
            if (!k) {
                k = item.get();
            } else if (!v) {
                v = item.get();
            } else {
                return error("too many values in key-value pair",
                             item->location());
            }
        }
        if (!(k && v)) {
            return error("too few values in key-value pair",
                         kvpair->location());
        }
        if (k->type() != document::Type::String) {
            return error(
                absl::StrCat("expecting string but found ", k->type_name()),
                k->location());
        }
        RETURN_IF_ERROR(r.add(k->as<document::String>()->value));
        ASSIGN_OR_RETURN(auto slot,
                         r.getitem(k->as<document::String>()->value));
        RETURN_IF_ERROR(deserialize(slot, v));
    }
    return absl::OkStatus();
}

absl::Status Deserializer::deserialize(Ref r, const Document* doc) {
    ASSIGN_OR_RETURN(auto hint, r.hint());
    switch (hint) {
        case ::types::TypeHint::Primitive:
            return primitive(r, doc);
        case ::types::TypeHint::Struct:
            return structure(r, doc);
        case ::types::TypeHint::Vector:
            return vector(r, doc);
        case ::types::TypeHint::Map:
            return map(r, doc);
        default:
            return absl::UnimplementedError("not implemented");
    }
}

}  // namespace ajson
