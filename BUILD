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
        "//imwidget:editor",
        "//imwidget:hwpalette",
        "//imwidget:neschrview",
        "//imwidget:simplemap",
        "//imwidget:start_values",
        "//imwidget:object_table",
        "//nes:cartridge",
        "//nes:cpu6502",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:fpsmgr",
        "//util:os",
        "//util:logging",
        "//external:gflags",
        "//external:imgui_sdl_opengl",
        "//external:nfd",
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
