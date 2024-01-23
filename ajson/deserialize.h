#ifndef EMPTY_PROJECT_AJSON_DESERIALIZE_H
#define EMPTY_PROJECT_AJSON_DESERIALIZE_H

#include <memory>

#include "absl/status/status.h"
#include "ajson/document.h"
#include "ajson/reflect.h"

namespace ajson {

class Deserializer {
  public:
    absl::Status deserialize(Ref r, const Document* doc);

  private:
    absl::Status primitive(Ref r, const Document* doc);
    absl::Status structure(Ref r, const Document* doc);
    absl::Status vector(Ref r, const Document* doc);
    absl::Status optional(Ref r, const Document* doc);
    absl::Status map(Ref r, const Document* doc);
};

}  // namespace ajson

#endif  // EMPTY_PROJECT_AJSON_DESERIALIZE_H
