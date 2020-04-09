package(default_visibility = ["//visibility:public"])

cc_library(
    name = "gamecontrollerdb",
    srcs = ["gamecontrollerdb.cc"],
    hdrs = ["gamecontrollerdb.h"],
)

cc_library(
    name = "browser",
    srcs = [
        "browser.cc",
    ],
    hdrs = [
        "browser.h",
    ],
)

cc_library(
    name = "compress",
    srcs = [
        "compress.cc",
    ],
    hdrs = [
        "compress.h",
    ],
    linkopts = ["-lz"],
    deps = [
        "//util:logging",
        "//util:status",
    ],
)

cc_library(
    name = "config",
    hdrs = [
        "config.h",
    ],
    deps = [
        "//util:file",
        "//util:logging",
        "//util:os",
    ],
)

cc_library(
    name = "crc",
    srcs = ["crc.cc"],
    hdrs = ["crc.h"],
    linkopts = ["-lz"],
)

cc_library(
    name = "file",
    srcs = [
        "file.cc",
    ],
    hdrs = [
        "file.h",
    ],
    deps = [
        ":status",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "logging",
    srcs = [
        "logging.cc",
    ],
    hdrs = [
        "logging.h",
    ],
    deps = [
        "//external:gflags",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "os",
    srcs = [
        "os.cc",
    ],
    hdrs = [
        "os.h",
    ],
    deps = [
        ":logging",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "status",
    srcs = [
        "status.cc",
    ],
    hdrs = [
        "status.h",
        "statusor.h",
    ],
    deps = [
        "@com_google_protobuf//:protobuf",
    ],
)

cc_library(
    name = "fpsmgr",
    hdrs = [
        "fpsmgr.h",
    ],
)

cc_library(
    name = "macros",
    hdrs = [
        "macros.h",
    ],
)

cc_library(
    name = "stb-tilemap-editor",
    hdrs = [
        "stb_tilemap_editor.h",
    ],
)

cc_library(
    name = "stb-perlin",
    hdrs = [
        "stb_perlin.h",
    ],
)

genrule(
    name = "gen_fontawesome_h",
    srcs = ["data/fontawesome-webfont.ttf"],
    outs = ["fontawesome.h"],
    cmd = "xxd -i $< $@",
)

cc_library(
    name = "imgui_sdl_opengl",
    srcs = [
        "fontawesome.h",
        "imgui_impl_sdl.cpp",
    ],
    hdrs = [
        "imgui_impl_sdl.h",
    ],
    deps = [
        "//external:fontawesome",
        "//external:imgui",
    ],
)

cc_binary(
    name = "chr_dump",
    srcs = ["chr_dump.cc"],
    deps = ["//:app"],
    linkopts = [
        "-lpthread",
        "-lm",
        "-lGL",
    ],
)
