package(default_visibility=["//visibility:public"])
#load(":proto.bzl", "proto_library")

proto_library(
    name = "rominfo_proto",
    srcs = [
        "rominfo.proto",
        "keybinds.proto",
    ],
)

cc_proto_library(
    name = "rominfo",
    deps = [":rominfo_proto"],
)

proto_library(
    name = "project_proto",
    srcs = [
        "project.proto",
    ],
    deps = [
        ":rominfo_proto",
    ],
)

cc_proto_library(
    name = "project",
    deps = [":project_proto"],
)
