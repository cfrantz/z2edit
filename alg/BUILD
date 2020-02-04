package(default_visibility = ["//visibility:public"])

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
    srcs = [
        "fdg.cc",
    ],
    hdrs = [
        "fdg.h",
    ],
    deps = [
        ":vecmath",
    ],
)

cc_library(
    name = "terrain",
    srcs = [
        "terrain.cc",
    ],
    hdrs = [
        "terrain.h",
    ],
    deps = [
        "//util:stb-perlin",
    ],
)

cc_library(
    name = "palace_gen",
    srcs = [
        "palace_gen.cc",
    ],
    hdrs = [
        "palace_gen.h",
    ],
    deps = [
        "//imwidget:simplemap",
        "//nes:mappers",
        "//proto:generator",
        "//proto:rominfo",
        "//util:config",
    ],
)
