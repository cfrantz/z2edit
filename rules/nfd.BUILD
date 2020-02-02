package(default_visibility = ["//visibility:public"])

config_setting(
    name = "windows",
    values = {
        "crosstool_top": "@mxebzl//compiler:win64",
    }
)

genrule(
    name = "fucking-glib-headers",
    outs = [
        "glib-2.0/include/glibconfig.h",
    ],
    cmd = """
    if [[ -f /usr/lib/x86_64-linux-gnu/glib-2.0/include/glibconfig.h ]]; then
        # Ubuntu, debian
        cp /usr/lib/x86_64-linux-gnu/glib-2.0/include/glibconfig.h $(@)
    elif [[ -f /usr/lib/glib-2.0/include/glibconfig.h ]]; then
        # Arch
        cp /usr/lib/glib-2.0/include/glibconfig.h $(@)
    fi
    """,
)

cc_library(
    name = "nfd-linux",
    copts = [
        "-I/usr/include/gtk-3.0",
        "-I/usr/include/gdk-pixbuf-2.0",
        "-I/usr/include/glib-2.0",
        "-I/usr/include/atk-1.0",
        "-I/usr/include/pango-1.0",
        "-I/usr/include/cairo",
        "-I/usr/include/harfbuzz",
    ],
    linkopts = [
        "-lgtk-3",
        "-lglib-2.0",
        "-lgdk-3",
        "-latk-1.0",
        "-lgio-2.0",
        "-lpangocairo-1.0",
        "-lgdk_pixbuf-2.0",
        "-lcairo-gobject",
        "-lpango-1.0",
        "-lcairo",
        "-lgobject-2.0",
        "-lglib-2.0",
    ],
    includes = [
        "src/include",
        "glib-2.0/include",
    ],
    hdrs = [
        "src/include/nfd.h",
    ],
    srcs = [
        "src/common.h",
        "src/nfd_common.h",
        "src/nfd_common.c",
        "src/nfd_gtk.c",
        "glib-2.0/include/glibconfig.h",
    ],
)

genrule(
    name = "fucking-windows-filenames",
    srcs = [ "src/nfd_win.cpp" ],
    outs = [ "src/nfd_win_fixed.cpp" ],
    cmd = "sed -e 's/ShObjIdl.h/shobjidl.h/g' $< >$(@)"
)

cc_library(
    name = "nfd-windows",
    defines = [ "MINGW_HAS_SECURE_API=1" ],
    includes = [
        "src",
        "src/include",
    ],
    hdrs = [
        "src/include/nfd.h",
    ],
    srcs = [
        "src/common.h",
        "src/nfd_common.h",
        "src/nfd_common.c",
        "src/nfd_win_fixed.cpp",
    ],
)

cc_library(
    name = "nfd",
    defines = [ "HAVE_NFD" ],
    deps = select({
        ":windows": [ ":nfd-windows" ],
        "//conditions:default": [ ":nfd-linux" ],
    }),
)
