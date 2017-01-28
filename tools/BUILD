package(default_visibility = ["//visibility:public"])

filegroup(
    name = "srcs",
    srcs = glob(["**"], exclude=["mxe/**"]) + [
        "//tools/windows:srcs",
    ],
)

cc_binary(
    name = "pack_config",
    srcs = ["pack_config.cc"],
    deps = [
        "//proto:rominfo",
        "//util:config",
        "//external:gflags",
    ],
)
