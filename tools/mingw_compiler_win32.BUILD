package(default_visibility = ['//visibility:public'])

filegroup(
    name = 'gcc',
    srcs = ['usr/bin/i686-w64-mingw32.shared-gcc'],
)

filegroup(
    name = 'addr2line',
    srcs = ['usr/bin/i686-w64-mingw32.shared-addr2line'],
)

filegroup(
    name = 'ar',
    srcs = ['usr/bin/i686-w64-mingw32.shared-ar'],
)

filegroup(
    name = 'as',
    srcs = ['usr/bin/i686-w64-mingw32.shared-as'],
)

filegroup(
    name = 'c++filt',
    srcs = ['usr/bin/i686-w64-mingw32.shared-c++filt'],
)

filegroup(
    name = 'dlltool',
    srcs = ['usr/bin/i686-w64-mingw32.shared-dlltool'],
)

filegroup(
    name = 'dllwrap',
    srcs = ['usr/bin/i686-w64-mingw32.shared-dllwrap'],
)

filegroup(
    name = 'elfedit',
    srcs = ['usr/bin/i686-w64-mingw32.shared-elfedit'],
)

filegroup(
    name = 'gprof',
    srcs = ['usr/bin/i686-w64-mingw32.shared-gprof'],
)

filegroup(
    name = 'ld',
    srcs = ['usr/bin/i686-w64-mingw32.shared-ld'],
)

filegroup(
    name = 'ld.bfd',
    srcs = ['usr/bin/i686-w64-mingw32.shared-ld.bfd'],
)

filegroup(
    name = 'nm',
    srcs = ['usr/bin/i686-w64-mingw32.shared-nm'],
)

filegroup(
    name = 'objcopy',
    srcs = ['usr/bin/i686-w64-mingw32.shared-objcopy'],
)

filegroup(
    name = 'objdump',
    srcs = ['usr/bin/i686-w64-mingw32.shared-objdump'],
)

filegroup(
    name = 'ranlib',
    srcs = ['usr/bin/i686-w64-mingw32.shared-ranlib'],
)

filegroup(
    name = 'readelf',
    srcs = ['usr/bin/i686-w64-mingw32.shared-readelf'],
)

filegroup(
    name = 'size',
    srcs = ['usr/bin/i686-w64-mingw32.shared-size'],
)

filegroup(
    name = 'strings',
    srcs = ['usr/bin/i686-w64-mingw32.shared-strings'],
)

filegroup(
    name = 'strip',
    srcs = ['usr/bin/i686-w64-mingw32.shared-strip'],
)

filegroup(
    name = 'windmc',
    srcs = ['usr/bin/i686-w64-mingw32.shared-windmc'],
)

filegroup(
    name = 'windres',
    srcs = ['usr/bin/i686-w64-mingw32.shared-windres'],
)

filegroup(
  name = 'compiler_pieces',
  srcs = glob([
    'usr/i686-w64-mingw32.shared/**',
    'usr/libexec/gcc/i686-w64-mingw32.shared/**',
    'usr/lib/gcc/i686-w64-mingw32.shared/**',
  ]),
)

filegroup(
    name = 'compiler_components',
    srcs = [
        ':gcc',
        ':addr2line',
        ':ar',
        ':as',
        ':c++filt',
        ':dlltool',
        ':dllwrap',
        ':elfedit',
        ':gprof',
        ':ld',
        ':ld.bfd',
        ':nm',
        ':objcopy',
        ':objdump',
        ':ranlib',
        ':readelf',
        ':size',
        ':strings',
        ':strip',
        ':windmc',
        ':windres',
    ],
)
