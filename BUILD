package(default_visibility=["//visibility:public"])

cc_library(
    name = "imapp",
    linkopts = [
        "-lSDL2",
        "-lSDL2_image",
        "-lSDL2_mixer",
        "-lSDL2_gfx",
    ],
    hdrs = [
        "imapp.h",
    ],
    srcs = [
        "imapp.cc",
    ],
    deps = [
        "//imwidget:debug_console",
        "//imwidget:hwpalette",
        "//imwidget:neschrview",
        "//imwidget:simplemap",
        "//nes:cartridge",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:fpsmgr",
        "//util:os",
        "//external:gflags",
        "//external:imgui_sdl_opengl",
    ],
)

cc_binary(
    name = "main",
    linkopts = [
        "-lpthread",
    ],
    srcs = [
        "main.cc",
    ],
    deps = [
        ":imapp",
        "//util:config",
        "//external:gflags",
    ],
)
