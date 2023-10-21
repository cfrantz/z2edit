#ifndef WVX_UTIL_PROTO_FILE_H
#define WVX_UTIL_PROTO_FILE_H

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "google/protobuf/message.h"

class ProtoFile {
    enum Type {
        Auto,
        Binary,
        Text,
    };
    static absl::Status Save(const std::string& filename,
                             const google::protobuf::Message& message,
                             Type type = Type::Auto);
    static absl::Status Load(const std::string& filename,
                             google::protobuf::Message* message,
                             Type type = Type::Auto);

    template <typename T>
    static absl::StatusOr<T> Load(const std::string& filename,
                                  Type type = Type::Auto) {
        T message;
        absl::Status sts = Load(filename, &message, type);
        if (!sts.ok()) return sts;
        return message;
    }
};

#endif  // WVX_UTIL_PROTO_FILE_H
