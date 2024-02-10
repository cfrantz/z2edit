#ifndef EMPTY_PROJECT_AJSON_REFLECT_H
#define EMPTY_PROJECT_AJSON_REFLECT_H
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
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
    std::string_view metadata;
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
        virtual size_t index() { return 0; }
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
        std::string name_;
        std::string_view type_;
    };

    /** A container for struct types. */
    template <typename T>
    class Structure : public _Ref {
      public:
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
        std::string name_;
        std::string_view type_;
    };

    /** A container for optional types. */
    template <typename T>
    class Optional : public _Ref {
      public:
        static Optional<T>* New(std::optional<T>& v, std::string_view name) {
            return new Optional(&v, name, ::types::Type::of_val(v));
        }
        absl::StatusOr<Ref> getitem(std::string_view key) override;
        absl::StatusOr<std::map<std::string, Annotation>> fields() override {
            static std::map<std::string, Annotation> value = {
                {"value", Annotation{}}};
            if (!ptr_->has_value()) {
                return value;
            } else {
                return std::map<std::string, Annotation>();
            }
        }
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override {
            return ::types::TypeHint::Optional;
        }
        size_t size() override { return ptr_->has_value(); }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) {
            if (key == "value") {
                if (ptr_->has_value())
                    return absl::AlreadyExistsError(
                        "value already in optional");
                ptr_->emplace();
            } else if (key == "null") {
                ptr_->reset();
            } else {
                return absl::InvalidArgumentError("bad key for optional");
            }
            return absl::OkStatus();
        }

      private:
        Optional(std::optional<T>* v, std::string_view name,
                 std::string_view type)
            : ptr_(v), name_(name), type_(type) {}
        std::optional<T>* ptr_;
        std::string name_;
        std::string_view type_;
    };

    /** A container for variant types. */
    template <typename... T>
    class Variant : public _Ref {
      public:
        static Variant<T...>* New(std::variant<T...>& v, std::string_view name,
                                  std::string_view metadata) {
            return new Variant(&v, name, ::types::Type::of_val(v), metadata);
        }
        absl::StatusOr<Ref> getitem(std::string_view key) override;
        absl::StatusOr<Ref> getitem(size_t index) override;
        absl::StatusOr<std::map<std::string, Annotation>> fields() override;
        std::string_view type() override { return type_; }
        std::string_view name() override { return name_; }
        ::types::TypeHint hint() override { return ::types::TypeHint::Variant; }
        size_t size() override {
            return std::variant_size_v<std::variant<T...>>;
        }
        size_t index() override { return ptr_->index(); }
        void* ptr() override { return ptr_; }
        absl::Status add(const std::string& key) override;

      private:
        Variant(std::variant<T...>* v, std::string_view name,
                std::string_view type, std::string_view metadata)
            : ptr_(v), type_(type) {
            if (!metadata.empty()) {
                if (metadata.starts_with("variant:")) {
                    fields_ = absl::StrSplit(metadata.substr(8), ',');
                } else {
                    LOG(ERROR) << "Variant '" << name
                               << "' has bad metadata: " << metadata;
                }
            }
        }
        std::variant<T...>* ptr_;
        std::string_view type_;
        std::string name_;
        std::vector<std::string> fields_;
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
        std::string name_;
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
        std::string name_;
        std::string_view type_;
    };

    std::shared_ptr<_Ref> ref_;
    Ref(_Ref* v) : ref_(v) {}

  public:
    /** Constructs a Ref for variant types. */
    template <typename... T>
    static Ref New(std::variant<T...>& v, std::string_view name,
                   std::string_view metadata = "") {
        return Ref(Variant<T...>::New(v, name, metadata));
    }

    /** Constructs a Ref for vector types. */
    template <typename T>
    static Ref New(std::vector<T>& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "Vector '" << name << "' cannot accept metadata.";
        return Ref(Vector<T>::New(v, name));
    }

    /** Constructs a Ref for optional types. */
    template <typename T>
    static Ref New(std::optional<T>& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "Optional '" << name << "' cannot accept metadata.";
        return Ref(Optional<T>::New(v, name));
    }

    /** Constructs a Ref for map types. */
    template <typename K, typename V>
    static Ref New(std::map<K, V>& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "Map '" << name << "' cannot accept metadata.";
        return Ref(Map<K, V>::New(v, name));
    }

    /** Constructs a Ref for struct types. */
    template <typename T>
    static Ref New(T& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "Structure '" << name << "' cannot accept metadata.";
        return Ref(Structure<T>::New(v, name));
    }

    static Ref New(bool& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "bool '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint8_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "uint8_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint16_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "uint16_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint32_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "uint32_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(uint64_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "uint64_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(int8_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "int8_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(int16_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "int16_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(int32_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "int32_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(int64_t& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "int64_t '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(float& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "float '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(double& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "double '" << name << "' cannot accept metadata.";
        return Ref(new Primitive(v, name));
    }
    static Ref New(std::string& v, std::string_view name,
                   std::string_view metadata = "") {
        if (!metadata.empty())
            LOG(ERROR) << "string '" << name << "' cannot accept metadata.";
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
    /** Gets the current index of a Ref (ie: variant). */
    absl::StatusOr<size_t> index() {
        if (!ref_) return absl::UnknownError("Ref: internal ref is nullptr");
        return ref_->index();
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

template <typename T>
absl::StatusOr<Ref> Ref::Structure<T>::getitem(std::string_view key) {
    return ptr_->_getitem(key);
}

template <typename T>
absl::StatusOr<std::map<std::string, Annotation>> Ref::Structure<T>::fields() {
    return ptr_->_fields();
}

template <typename T>
absl::StatusOr<Ref> Ref::Vector<T>::getitem(std::string_view key) {
    size_t index = 0;
    if (absl::SimpleAtoi(key, &index)) {
        return getitem(index);
    } else {
        return absl::NotFoundError(absl::StrCat("bad key: ", key));
    }
}

template <typename T>
absl::StatusOr<Ref> Ref::Vector<T>::getitem(size_t index) {
    if (index < ptr_->size()) {
        return Ref::New(ptr_->at(index), absl::StrCat(index));
    } else {
        return absl::NotFoundError(absl::StrCat("bad index: ", index));
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

template <typename T>
absl::StatusOr<Ref> Ref::Optional<T>::getitem(std::string_view key) {
    if (!ptr_->has_value()) {
        return absl::NotFoundError("no value in optional");
    } else if (key != "value") {
        return absl::InvalidArgumentError("bad key for optional");
    } else {
        return Ref::New(ptr_->value(), "value");
    }
}

#include "ajson/variant_helpers.h"

template <typename... T>
absl::StatusOr<Ref> Ref::Variant<T...>::getitem(std::string_view key) {
    size_t index = 0;
    if (key == "*") {
        return getitem(SIZE_MAX);
    } else if (absl::SimpleAtoi(key, &index)) {
        return getitem(index);
    } else {
        for (; index < fields_.size(); ++index) {
            if (key == fields_[index]) {
                return getitem(index);
            }
        }
    }
    return absl::NotFoundError(absl::StrCat("unknown variant: ", key));
}

template <typename... T>
absl::StatusOr<Ref> Ref::Variant<T...>::getitem(size_t index) {
    if (index == SIZE_MAX) index = ptr_->index();
    std::string key;
    if (index < fields_.size()) {
        key = fields_[index];
    } else {
        LOG(ERROR) << "Variant '" << name_
                   << "' does not have a name for variant " << index;
        key = absl::StrCat(index);
    }
    return internal::VariantHelper<
        std::variant_size_v<std::variant<T...>>>::get(index, key, *ptr_);
}

template <typename... T>
absl::StatusOr<std::map<std::string, Annotation>> Ref::Variant<T...>::fields() {
    std::map<std::string, Annotation> fields;
    return fields;
}

template <typename... T>
absl::Status Ref::Variant<T...>::add(const std::string& key) {
    size_t index = 0;
    if (absl::SimpleAtoi(key, &index)) {
        return internal::VariantHelper<
            std::variant_size_v<std::variant<T...>>>::emplace(index, *ptr_);
    } else {
        for (; index < fields_.size(); ++index) {
            if (key == fields_[index]) {
                return internal::VariantHelper<
                    std::variant_size_v<std::variant<T...>>>::emplace(index,
                                                                      *ptr_);
            }
        }
        LOG(FATAL) << "Variant '" << name_ << "': unknown variant '" << key
                   << "'.";
    }
    return absl::NotFoundError(absl::StrCat("unknown variant: ", key));
}

}  // namespace ajson
#endif  // EMPTY_PROJECT_AJSON_REFLECT_H
