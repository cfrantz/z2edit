package(default_visibility=["//visibility:public"])

cc_library(
    name = "base",
    hdrs = [
        "debug_console.h",
        "imapp.h",
        "imutil.h",
        "imwidget.h",
    ],
    srcs = [
        "debug_console.cc",
        "imapp.cc",
        "imutil.cc",
    ],
    deps = [
        "//util:fpsmgr",
        "//util:imgui_sdl_opengl",
        "//util:logging",
        "//external:gflags",
        "//external:imgui",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "error_dialog",
    hdrs = ["error_dialog.h"],
    srcs = ["error_dialog.cc"],
    deps = [
        ":base",
        "//external:imgui",
        "@com_google_absl//absl/strings",
    ],
)

cc_library(
    name = "glbitmap",
    hdrs = ["glbitmap.h"],
    srcs = ["glbitmap.cc"],
    deps = [
        "//external:imgui",
    ],
)
