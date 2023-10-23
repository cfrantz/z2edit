#include "util/proto_file.h"

#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>

#include <fstream>
#include <iostream>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "google/protobuf/io/zero_copy_stream_impl.h"
#include "google/protobuf/message.h"
#include "google/protobuf/text_format.h"
#include "util/posix_status.h"

absl::Status ProtoFile::Save(const std::string& filename,
                             const google::protobuf::Message& message,
                             Type type) {
    if (type == Type::Auto) {
        if (absl::EndsWith(filename, ".textpb") ||
            absl::EndsWith(filename, ".textproto") ||
            absl::EndsWith(filename, ".txt")) {
            type = Type::Text;
        } else {
            type = Type::Binary;
        }
    }
    if (type == Type::Text) {
        int fd = open(filename.c_str(), O_WRONLY | O_TRUNC | O_CREAT);
        if (fd == -1) {
            return util::PosixStatus(errno);
        }
        google::protobuf::io::FileOutputStream out(fd);
        if (!google::protobuf::TextFormat::Print(message, &out)) {
            return absl::InternalError("text serialization failed");
        }
        close(fd);
        return absl::OkStatus();
    } else if (type == Type::Binary) {
        std::fstream out(filename, std::ios_base::out | std::ios_base::trunc |
                                       std::ios_base::binary);
        if (!message.SerializeToOstream(&out)) {
            return absl::InternalError("serialization failed");
        }
        return absl::OkStatus();
    } else {
        return absl::InvalidArgumentError("unknown file type");
    }
}

absl::Status ProtoFile::Load(const std::string& filename,
                             google::protobuf::Message* message, Type type) {
    if (type == Type::Auto) {
        if (absl::EndsWith(filename, ".textpb") ||
            absl::EndsWith(filename, ".textproto") ||
            absl::EndsWith(filename, ".txt")) {
            type = Type::Text;
        } else {
            type = Type::Binary;
        }
    }
    if (type == Type::Text) {
        int fd = open(filename.c_str(), O_RDONLY);
        if (fd == -1) {
            return util::PosixStatus(errno);
        }
        google::protobuf::io::FileInputStream inp(fd);
        if (!google::protobuf::TextFormat::Parse(&inp, message)) {
            return absl::InternalError("text deserialization failed");
        }
        close(fd);
        return absl::OkStatus();
    } else if (type == Type::Binary) {
        std::fstream inp(filename, std::ios_base::in | std::ios_base::binary);
        if (!message->ParseFromIstream(&inp)) {
            return absl::InternalError("deserialization failed");
        }
        return absl::OkStatus();
    } else {
        return absl::InvalidArgumentError("unknown file type");
    }
}
