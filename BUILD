package(default_visibility=["//visibility:public"])

cc_library(
    name = "imapp-util",
    hdrs = ["imapp-util.h"],
    srcs = [],
)

cc_library(
    name = "nfd",
    defines = [ "HAVE_NFD" ],
    deps = [
        "//external:nfd",
    ]
)

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
        ":imapp-util",
        # TODO(cfrantz): on ubuntu 16 with MIR, there is a library conflict
        # between MIR (linked with protobuf 2.6.1) and this program,
        # which builds with protbuf 3.x.x.  A temporary workaround is to
        # not link with nfd (native-file-dialog).
        ":nfd",
        "//imwidget:debug_console",
        "//imwidget:editor",
        "//imwidget:hwpalette",
        "//imwidget:misc_hacks",
        "//imwidget:neschrview",
        "//imwidget:simplemap",
        "//imwidget:start_values",
        "//imwidget:object_table",
        "//nes:cartridge",
        "//nes:cpu6502",
        "//nes:mappers",
        "//proto:rominfo",
        "//util:fpsmgr",
        "//util:imgui_sdl_opengl",
        "//util:os",
        "//util:logging",
        "//external:gflags",
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
