package(default_visibility = ["//visibility:public"])

genrule(
    name = "fucking-glib-headers",
    outs = [
        "glib-2.0/include/glibconfig.h",
    ],
    cmd = "cp /usr/lib/x86_64-linux-gnu/glib-2.0/include/glibconfig.h $(@)"
)

cc_library(
    name = "nfd",
    copts = [
        "-I/usr/include/gtk-3.0",
        "-I/usr/include/gdk-pixbuf-2.0",
        "-I/usr/include/glib-2.0",
        "-I/usr/include/atk-1.0",
        "-I/usr/include/pango-1.0",
        "-I/usr/include/cairo",
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
