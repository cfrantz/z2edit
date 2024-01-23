#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include "ajson/relax.h"
#include "ajson/document.h"
#include <cstdint>

namespace py = pybind11;
namespace ajson {

using namespace document;

class DocIterator {
  public:
    DocIterator(const std::vector<std::shared_ptr<Document>> doc, py::object ref)
      : doc_(doc), ref_(ref) {}

    const std::shared_ptr<Document> next() {
        if (index_ == doc_.size())
            throw py::stop_iteration();
        return doc_[index_++];
    }

  private:
    const std::vector<std::shared_ptr<Document>> doc_;
    py::object ref_;
    size_t index_ = 0;
};


#define VAL(x) \
  [](py::object /*self*/) { return static_cast<uint32_t>(x); }

PYBIND11_MODULE(_relax, m) {
    auto doc_class = py::class_<Document, std::shared_ptr<Document>>(m, "Document");

    py::class_<DocIterator>(m, "DocIterator")
        .def("__iter__", [](DocIterator& it) -> DocIterator& { return it; })
        .def("__next__", &DocIterator::next);

    py::enum_<CommentFormat>(m, "CommentFormat")
        .value("NONE",       CommentFormat::None)
        .value("BLOCK",      CommentFormat::Block)
        .value("HASH",       CommentFormat::Hash)
        .value("SLASHSLASH", CommentFormat::SlashSlash)
        .export_values()
        ;

    py::enum_<StringFormat>(m, "StringFormat")
        .value("NONE",     StringFormat::None)
        .value("QUOTED",   StringFormat::Quoted)
        .value("UNQUOTED", StringFormat::Unquoted)
        .value("BLOCK",    StringFormat::Block)
        .export_values()
        ;

    py::enum_<Base>(m, "Base")
        .value("BINARY",  Base::Binary)
        .value("OCTAL",   Base::Octal)
        .value("DECIMAL", Base::Decimal)
        .value("HEX",     Base::Hex)
        .export_values()
        ;

    py::enum_<Type>(m, "Type")
        .value("COMMENT",  Type::Comment)
        .value("STRING",   Type::String)
        .value("BOOLEAN",  Type::Boolean)
        .value("INT",      Type::Int)
        .value("REAL",     Type::Real)
        .value("MAPPING",  Type::Mapping)
        .value("SEQUENCE", Type::Sequence)
        .value("BYTES",    Type::Bytes)
        .value("NULL",     Type::Null)
        .value("COMPACT",  Type::Compact)
        .value("FRAGMENT", Type::Fragment)
        .export_values()
        ;

    py::class_<Location>(m, "Location")
        .def_readonly("line", &Location::line)
        .def_readonly("column", &Location::column)
        .def("__str__", &Location::location)
        ;


    py::class_<Comment>(m, "Comment")
        .def_readonly("value", &Comment::value)
        .def_readonly("format", &Comment::format)
        .def_readonly("location", &Comment::location)
        ;

    py::class_<String>(m, "String")
        .def_readonly("value", &String::value)
        .def_readonly("format", &String::format)
        .def_readonly("location", &String::location)
        ;

    py::class_<Boolean>(m, "Boolean")
        .def_readonly("value", &Boolean::value)
        .def_readonly("location", &Boolean::location)
        ;

    py::class_<Int>(m, "Int")
        .def_readonly("value", &Int::value)
        .def_readonly("negative", &Int::negative)
        .def_readonly("size", &Int::size)
        .def_readonly("location", &Int::location)
        .def("__int__", [](Int* self) { return self->as<int64_t>(); })
        ;

    py::class_<Real>(m, "Real")
        .def_readonly("value", &Real::value)
        .def_readonly("location", &Real::location)
        ;

    py::class_<Null>(m, "Null")
        .def_readonly("location", &Null::location)
        ;

    py::class_<Mapping>(m, "Mapping")
        .def_readonly("location", &Mapping::location)
        .def_property_readonly("value", [](py::object self) {
            return DocIterator(self.cast<const Mapping&>().value, self);
        }, py::keep_alive<0, 1>())
        ;

    py::class_<Sequence>(m, "Sequence")
        .def_readonly("location", &Sequence::location)
        .def_property_readonly("value", [](py::object self) {
            return DocIterator(self.cast<const Sequence&>().value, self);
        }, py::keep_alive<0, 1>())
        ;
    py::class_<Compact>(m, "Compact")
        .def_readonly("location", &Compact::location)
        .def_property_readonly("value", [](py::object self) {
            return DocIterator(self.cast<const Compact&>().value, self);
        }, py::keep_alive<0, 1>())
        ;
    py::class_<Fragment>(m, "Fragment")
        .def_readonly("location", &Fragment::location)
        .def_property_readonly("value", [](py::object self) {
            return DocIterator(self.cast<const Fragment&>().value, self);
        }, py::keep_alive<0, 1>())
        ;

#define DOC_AS(x) \
    [](Document* self) { \
        const auto val = self->as<x>(); \
        if (!val.ok()) { \
            std::string message(val.status().message()); \
            throw py::type_error(message); \
        } \
        return &*val; \
    }

    doc_class
        .def("type", &Document::type)
        .def("type_name", &Document::type_name)
        .def("structure", &Document::structure)
        .def("has_value", &Document::has_value)
        .def("location", &Document::location)
        .def_property_readonly("comment", DOC_AS(Comment))
        .def_property_readonly("string", DOC_AS(String))
        .def_property_readonly("boolean", DOC_AS(Boolean))
        .def_property_readonly("integer", DOC_AS(Int))
        .def_property_readonly("real", DOC_AS(Real))
        .def_property_readonly("null", DOC_AS(Null))
        .def_property_readonly("bytes", DOC_AS(Bytes))
        .def_property_readonly("mapping", DOC_AS(Mapping))
        .def_property_readonly("sequence", DOC_AS(Sequence))
        .def_property_readonly("compact", DOC_AS(Compact))
        .def_property_readonly("fragment", DOC_AS(Fragment))
        .def("info", [](Document *self) {
            printf("self = %p\n", self);
        })
        ;

    py::class_<Relax>(m, "Relax")
        .def(py::init<>())
        .def("parse", [](Relax* self, const std::string& str) {
            auto val = self->parse(str);
            if (!val.ok()) {
                std::string message(val.status().message());
                throw std::invalid_argument(message);
            }
            return *val;
        });
}

}  // namespace ajson
