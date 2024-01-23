#ifndef EMPTY_PROJECT_AJSON_SERIALIZE_H
#define EMPTY_PROJECT_AJSON_SERIALIZE_H

#include <memory>

#include "absl/status/statusor.h"
#include "ajson/document.h"
#include "ajson/reflect.h"

namespace ajson {

class Serializer {
  public:
    absl::StatusOr<std::shared_ptr<Document>> serialize(Ref r);

  private:
    absl::StatusOr<std::shared_ptr<Document>> primitive(Ref r);
    absl::StatusOr<std::shared_ptr<Document>> structure(Ref r);
    absl::StatusOr<std::shared_ptr<Document>> vector(Ref r);
    absl::StatusOr<std::shared_ptr<Document>> optional(Ref r);
    absl::StatusOr<std::shared_ptr<Document>> map(Ref r);
    struct State {
        bool compact = false;
        Base base = Base::Decimal;
        StringFormat str = StringFormat::None;
    };
    State apply_annotation(const Annotation& a);
    State state_;
};

}  // namespace ajson
#endif  // EMPTY_PROJECT_AJSON_SERIALIZE_H
