package(default_visibility=["//visibility:public"])

cc_library(
    name = "vecmath",
    hdrs = [
        "vec2.h",
    ],
    deps = [
        "//external:imgui",
    ],
)

cc_library(
    name = "fdg",
    hdrs = [
        "fdg.h",
    ],
    srcs = [
        "fdg.cc",
    ],
    deps = [
        ":vecmath",
    ],
)
