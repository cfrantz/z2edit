package(default_visibility=["//visibility:public"])
load(":proto.bzl", "proto_library")

proto_library(
    name = "rominfo",
    srcs = [
        "rominfo.proto",
    ],
)

proto_library(
    name = "project",
    srcs = [
        "project.proto",
    ],
    deps = [
        ":rominfo",
    ],
)
