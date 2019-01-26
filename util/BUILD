package(default_visibility=["//visibility:public"])

cc_library(
    name = "gamecontrollerdb",
    hdrs = ["gamecontrollerdb.h"],
    srcs = ["gamecontrollerdb.cc"],
)

cc_library(
    name = "browser",
    hdrs = [
        "browser.h",
    ],
    srcs = [
        "browser.cc",
    ],
)

cc_library(
    name = "compress",
    linkopts = ["-lz"],
    hdrs = [
        "compress.h",
    ],
    srcs = [
        "compress.cc",
    ],
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
    linkopts = ["-lz"],
    hdrs = ["crc.h"],
    srcs = ["crc.cc"],
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
    ]
)

cc_library(
    name = "logging",
    hdrs = [
        "logging.h",
    ],
    srcs = [
        "logging.cc",
    ],
    deps = [
        "@com_google_absl//absl/strings",
        "//external:gflags",
    ]
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
    ]
)

cc_library(
    name = "status",
    hdrs = [
        "status.h",
        "statusor.h",
    ],
    srcs = [
        "status.cc",
    ],
    deps = [
        "@com_google_protobuf//:protobuf",
    ]
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
    ]
)

cc_library(
    name = "stb-perlin",
    hdrs = [
        "stb_perlin.h",
    ]
)

genrule(
    name = "gen_fontawesome_h",
    srcs = ["data/fontawesome-webfont.ttf"],
    outs = ["fontawesome.h"],
    cmd = "xxd -i $< $@",
)

cc_library(
    name = "imgui_sdl_opengl",
    hdrs = [
        "imgui_impl_sdl.h",
    ],
    srcs = [
        "imgui_impl_sdl.cpp",
        "fontawesome.h",
    ],
    deps = [
        "//external:imgui",
        "//external:fontawesome",
    ],
)
