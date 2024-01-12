#ifndef EMPTY_PROJECT_AJSON_REFLECT_H
#define EMPTY_PROJECT_AJSON_REFLECT_H
#include <cstdint>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_split.h"
#include "ajson/reflect_macros.h"
#include "ajson/types.h"
#include "util/status.h"

namespace ajson {

/**
 * Output formatting options
 */
typedef uint32_t Format;
namespace format {
// clang-format off
constexpr Format None     = 0;
constexpr Format Compact  = 0x00000001;
constexpr Format Binary   = 0x00000002;
constexpr Format Decimal  = 0x0000000A;
constexpr Format Octal    = 0x00000008;
constexpr Format Hex      = 0x00000010;
constexpr Format HexStr   = 0x00000100;
constexpr Format Base64   = 0x00000200;
constexpr Format Hexdump  = 0x00000400;
constexpr Format Xxd      = 0x00000800;
constexpr Format Quoted   = 0x00001000;
constexpr Format Unquoted = 0x00002000;
constexpr Format Block    = 0x00003000;
// clang-format on
}  // namespace format

/**
 * Type annotations that get built into reflectable classes
 */
struct Annotation {
    std::string_view type;
    std::string_view comment;
    Format format{};
};

class Reflection;

/**
 * A type-erased reference object for accessing data via reflection.
 */
class Ref {
  private:
    /** The generic type-erased interface */
    class _Ref {
      public:
        virtual absl::StatusOr<Ref> getitem(std::string_view key) = 0;
        virtual absl::StatusOr<Ref> getitem(size_t index) {
            return absl::UnimplementedError("Integer indexing not supported");
        }
        virtual absl::StatusOr<std::map<std::string, Annotation>> fields() = 0;
        virtual std::string_view type() = 0;
        virtual std::string_view name() = 0;
        virtual ::types::TypeHint hint() = 0;
        virtual size_t size() { return 0; }
        virtual void* ptr() = 0;
        virtual absl::Status add(const std::string& key) = 0;
    };

    /** A container for primitive types. */
    class Primitive : public _Ref {
      public:
        Primitive(bool& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(uint8_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(uint16_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(uint32_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(uint64_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(int8_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(int16_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(int32_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(int64_t& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(float& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(double& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}
        Primitive(std::string& v, std::string_view name)
            : ptr_(&v), name_(name), type_(::types::Type::of_val(v)) {}

        absl::StatusOr<Ref> getitem(std::string_view key) override {
            return absl::UnimplementedError(
                "primitives don't support reflection");
        }
        absl::StatusOr<std::map<std::string, Annotation>> fields() override {
            return std::map<std::string, Annotation>();
        }
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override {
            return ::types::TypeHint::Primitive;
        }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) override {
            return absl::UnimplementedError("primitives aren't containers");
        }

      private:
        void* ptr_;
        std::string_view name_;
        std::string_view type_;
    };

    /** A container for struct types. */
    class Structure : public _Ref {
      public:
        template <typename T>
        static Structure* New(T& v, std::string_view name) {
            return new Structure(&v, name, ::types::Type::of_val(v));
        }

        absl::StatusOr<Ref> getitem(std::string_view key) override;
        absl::StatusOr<std::map<std::string, Annotation>> fields() override;
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override { return ::types::TypeHint::Struct; }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) override {
            return absl::UnimplementedError("structures are fixed containers");
        }

      private:
        Structure(Reflection* v, std::string_view name, std::string_view type)
            : ptr_(v), name_(name), type_(type) {}
        Reflection* ptr_;
        std::string_view name_;
        std::string_view type_;
    };

    /** A container for vector types. */
    template <typename T>
    class Vector : public _Ref {
      public:
        static Vector<T>* New(std::vector<T>& v, std::string_view name) {
            return new Vector(&v, name, ::types::Type::of_val(v));
        }
        absl::StatusOr<Ref> getitem(std::string_view key) override;
        absl::StatusOr<Ref> getitem(size_t index) override;
        absl::StatusOr<std::map<std::string, Annotation>> fields() override;
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override { return ::types::TypeHint::Vector; }
        size_t size() override { return ptr_->size(); }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) override {
            ptr_->emplace_back();
            return absl::OkStatus();
        }

      private:
        Vector(std::vector<T>* v, std::string_view name, std::string_view type)
            : ptr_(v), name_(name), type_(type) {}
        std::vector<T>* ptr_;
        std::string_view name_;
        std::string_view type_;
    };

    /** A container for map types. */
    template <typename K, typename V>
    class Map : public _Ref {
      public:
        static Map<K, V>* New(std::map<K, V>& v, std::string_view name) {
            return new Map(&v, name, ::types::Type::of_val(v));
        }
        absl::StatusOr<Ref> getitem(std::string_view key) override;
        absl::StatusOr<std::map<std::string, Annotation>> fields() override;
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override { return ::types::TypeHint::Map; }
        size_t size() override { return ptr_->size(); }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) override {
            ASSIGN_OR_RETURN(auto k, ::types::Type::from_str<K>(key));
            ptr_->try_emplace(k);
            return absl::OkStatus();
        }

      private:
        Map(std::map<K, V>* v, std::string_view name, std::string_view type)
            : ptr_(v), name_(name), type_(type) {}
        std::map<K, V>* ptr_;
        std::map<std::string, Annotation> keys_;
        std::string_view name_;
        std::string_view type_;
    };

    std::shared_ptr<_Ref> ref_;
    Ref(_Ref* v) : ref_(v) {}

  public:
    /** Constructs a Ref for vector types. */
    template <typename T>
    static Ref New(std::vector<T>& v, std::string_view name) {
        return Ref(Vector<T>::New(v, name));
    }

    /** Constructs a Ref for map types. */
    template <typename K, typename V>
    static Ref New(std::map<K, V>& v, std::string_view name) {
        return Ref(Map<K, V>::New(v, name));
    }

    /** Constructs a Ref for struct types. */
    template <typename T>
    static Ref New(T& v, std::string_view name) {
        return Ref(Structure::New(v, name));
    }

    static Ref New(bool& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint8_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint16_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint32_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint64_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(int8_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(int16_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(int32_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(int64_t& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(float& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(double& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }
    static Ref New(std::string& v, std::string_view name) {
        return Ref(new Primitive(v, name));
    }

#if 0
    /** Returns whether the Ref is in an Ok or Error state. */
    bool ok() const {
        if (!status_.ok()) return false;
        if (!ref_) return false;
        return true;
    }
    /** Returns the Ref's status object */
    absl::Status status() const {
        if (!status_.ok()) return status_;
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return absl::OkStatus();
    }
#endif

    /**
     * Returns a reference to the type-erased object or an error.
     *
     * You must know the type of the referenced object and provide it as
     * a template type parameter.
     */
    template <typename T>
    StatusOrRef<T> value() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        auto ty = ::types::Type::name<T>();
        if (ty != ref_->type()) {
            return absl::InvalidArgumentError(absl::StrCat(
                ref_->name(), ": cannot convert ", ref_->type(), " to ", ty));
        }
        return reinterpret_cast<T*>(ref_->ptr());
    }
    /**
     * Gets an item from the instance.
     *
     * @param path An object path to the desired item.
     */
    absl::StatusOr<Ref> get(std::string_view path) {
        path = path.substr(path.find_first_not_of("/"));
        std::vector<std::string_view> keys = absl::StrSplit(path, '/');
        auto k = keys.cbegin();
        auto stsval = getitem(*k);
        if (!stsval.ok()) return stsval.status();
        Ref value = *stsval;
        for (k++; k != keys.cend(); k++) {
            auto stsval = value.getitem(*k);
            if (!stsval.ok()) return stsval.status();
            value = *stsval;
        }
        return value;
    }
    /** Gets an item from a non-primitive Ref. */
    absl::StatusOr<Ref> getitem(std::string_view key) {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->getitem(key);
    }
    /** Gets an item from a non-primitive Ref. */
    absl::StatusOr<Ref> getitem(size_t index) {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->getitem(index);
    }
    /** Gets the list of fields known to the non-primitive Ref. */
    absl::StatusOr<std::map<std::string, Annotation>> fields() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->fields();
    }
    /** Gets name of the type of the Ref. */
    absl::StatusOr<std::string_view> type() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->type();
    }
    /** Gets name of the type hint of the Ref. */
    absl::StatusOr<::types::TypeHint> hint() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->hint();
    }
    /** Gets the size of a indexable Ref (ie: vector). */
    absl::StatusOr<size_t> size() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->size();
    }
    /** Gets field name of the Ref. */
    absl::StatusOr<std::string_view> name() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->name();
    }
    /** Adds a field to the Ref. */
    absl::Status add(const std::string& key) {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->add(key);
    }
};

/**
 * Reflection base class for all non-primitive types.
 */
class Reflection {
  public:
    /** Gets an Ref for this instance. */
    virtual Ref _ref() = 0;
    /** Gets an item from the instance. */
    virtual absl::StatusOr<Ref> _getitem(std::string_view key) = 0;
    /** Gets the list of fields known for this type. */
    virtual std::map<std::string, Annotation> _fields() = 0;
    /**
     * Gets an item from the instance.
     *
     * @param path An object path to the desired item.
     */
    absl::StatusOr<Ref> _get(std::string_view path) {
        path = path.substr(path.find_first_not_of("/"));
        std::vector<std::string_view> keys = absl::StrSplit(path, '/');
        auto k = keys.cbegin();
        auto stsval = _getitem(*k);
        if (!stsval.ok()) return stsval.status();
        Ref value = *stsval;
        for (k++; k != keys.cend(); k++) {
            auto stsval = value.getitem(*k);
            if (!stsval.ok()) return stsval.status();
            value = *stsval;
        }
        return value;
    }
};

absl::StatusOr<Ref> Ref::Structure::getitem(std::string_view key) {
    return ptr_->_getitem(key);
}
absl::StatusOr<std::map<std::string, Annotation>> Ref::Structure::fields() {
    return ptr_->_fields();
}

template <typename T>
absl::StatusOr<Ref> Ref::Vector<T>::getitem(std::string_view key) {
    size_t index = 0;
    if (absl::SimpleAtoi(key, &index)) {
        return getitem(index);
    } else {
        return absl::InvalidArgumentError(
            absl::StrCat("Bad key for Vector: ", key));
    }
}

template <typename T>
absl::StatusOr<Ref> Ref::Vector<T>::getitem(size_t index) {
    if (index < ptr_->size()) {
        return Ref::New(ptr_->at(index), absl::StrCat(index));
    } else {
        return absl::InvalidArgumentError(
            absl::StrCat("Bad index for Vector: ", index));
    }
}

template <typename T>
absl::StatusOr<std::map<std::string, Annotation>> Ref::Vector<T>::fields() {
    return std::map<std::string, Annotation>();
}

template <typename K, typename V>
absl::StatusOr<Ref> Ref::Map<K, V>::getitem(std::string_view key) {
    // FIXME: deal with non-string keys.
    auto k = ::types::Type::from_str<K>(key);
    if (!k.ok()) return k.status();
    auto v = ptr_->find(*k);
    if (v == ptr_->end()) {
        return absl::InvalidArgumentError(
            absl::StrCat("Bad key for Map: ", key));
    }
    return Ref::New(v->second, absl::StrCat(key));
}

template <typename K, typename V>
absl::StatusOr<std::map<std::string, Annotation>> Ref::Map<K, V>::fields() {
#if 1
    // FIXME: deal with non-string keys.
    keys_.clear();
    for (const auto& item : *ptr_) {
        auto key = ::types::Type::to_str(item.first);
        if (!key.ok()) return key.status();
        keys_.emplace(
            std::pair(*key, Annotation{.type = ::types::Type::name<V>()}));
    }
    return keys_;
#else
    static std::map<std::string_view, Annotation> none;
    return &none;
#endif
}

}  // namespace ajson
#endif  // EMPTY_PROJECT_AJSON_REFLECT_H
