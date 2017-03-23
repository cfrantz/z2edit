package(default_visibility=["//visibility:public"])

config_setting(
    name = "windows",
    values = {
        "crosstool_top": "//tools/windows:toolchain",
    }
)

genrule(
    name = "make_version",
    outs = ["version.h"],
    cmd = """sed -e 's/ / "/g' -e 's/$$/"\\n/g' -e "s/^/#define /g" bazel-out/volatile-status.txt > $@""",
    stamp = 1,
)

cc_library(
    name = "app",
    linkopts = [
        "-lSDL2main",
        "-lSDL2",
        "-lSDL2_image",
        "-lSDL2_mixer",
        "-lSDL2_gfx",
    ],
    hdrs = [
        "app.h",
        "version.h",
    ],
    srcs = [
        "app.cc",
    ],
    deps = [
        "//imwidget:base",
        "//imwidget:editor",
        "//imwidget:enemyattr",
        "//imwidget:hwpalette",
        "//imwidget:misc_hacks",
        "//imwidget:neschrview",
        "//imwidget:palace_gfx",
        "//imwidget:palette",
        "//imwidget:simplemap",
        "//imwidget:start_values",
        "//imwidget:object_table",
        "//imwidget:xptable",
        "//nes:cartridge",
        "//nes:cpu6502",
        "//nes:mappers",
        "//nes:text_encoding",
        "//proto:rominfo",
        "//util:browser",
        "//util:fpsmgr",
        "//util:imgui_sdl_opengl",
        "//util:os",
        "//util:logging",
        "//external:gflags",

        # TODO(cfrantz): on ubuntu 16 with MIR, there is a library conflict
        # between MIR (linked with protobuf 2.6.1) and this program,
        # which builds with protbuf 3.x.x.  A temporary workaround is to
        # not link with nfd (native-file-dialog).
        "//external:nfd",
    ],
)

genrule(
    name = "make_zelda2_config",
    srcs = ["zelda2.textpb"] + glob(["content/*.textpb"]),
    outs = ["zelda2_config.h"],
    cmd = "$(location //tools:pack_config) --config $(location zelda2.textpb)" +
          " --symbol kZelda2Cfg > $(@)",
    tools = ["//tools:pack_config"],
)


cc_binary(
    name = "z2edit",
    linkopts = select({
        ":windows": [
            "-lpthread",
            "-lm",
            "-lopengl32",
            "-ldinput8",
            "-ldxguid",
            "-ldxerr8",
            "-luser32",
            "-lgdi32",
            "-lwinmm",
            "-limm32",
            "-lole32",
            "-loleaut32",
            "-lshell32",
            "-lversion",
            "-luuid",

        ],
        "//conditions:default": [
            "-lpthread",
            "-lm",
            "-lGL",
        ],
    }),
    srcs = [
        "zelda2_config.h",
        "main.cc",
    ],
    deps = [
        ":app",
        "//util:config",
        "//external:gflags",
    ],
)
