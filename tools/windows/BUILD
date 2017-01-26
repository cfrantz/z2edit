# This is the entry point for --crosstool_top.  Toolchains are found
# by lopping off the name of --crosstool_top and searching for
# 'cc-compiler-${CPU}' in this BUILD file, where CPU is the target CPU
# specified in --cpu.
#
# This file group should include
#   * all cc_toolchain targets supported
#   * all file groups that said cc_toolchain might refer to,
# including the default_grte_top setting in the CROSSTOOL
# protobuf.
#
package(default_visibility = ["//visibility:public"])

filegroup(
  name = 'empty',
  srcs = [],
)

cc_toolchain_suite(
    name = 'toolchain',
    toolchains = {
        'win32|compiler': 'cc-compiler-win32',
        'win64|compiler': 'cc-compiler-win64',
        'local|compiler': 'cc-compiler-local',
    },
)

filegroup(
  name = 'mingw_all_files_win32',
  srcs = [
    '//tools/windows/win32:tool-wrappers',
    '@mingw_compiler_win32//:compiler_pieces',
  ],
)

filegroup(
  name = 'mingw_linker_files_win32',
  srcs = [
    '//tools/windows/win32:gcc',
    '//tools/windows/win32:ld',
    '//tools/windows/win32:ar',
    '@mingw_compiler_win32//:compiler_pieces',
  ],
)

filegroup(
  name = 'mingw_compiler_files_win32',
  srcs = [
    '//tools/windows/win32:gcc',
    '//tools/windows/win32:ld',
    '//tools/windows/win32:as',
    '@mingw_compiler_win32//:compiler_pieces',
  ],
)

cc_toolchain(
  name = 'cc-compiler-win32',
  all_files = ':mingw_all_files_win32',
  compiler_files = ':mingw_compiler_files_win32',
  cpu = 'win32',
  dwp_files = ':empty',
  dynamic_runtime_libs = [':empty'],
  linker_files = ':mingw_linker_files_win32',
  objcopy_files = '//tools/windows/win32:objcopy',
  static_runtime_libs = [':empty'],
  strip_files = '//tools/windows/win32:strip',
  supports_param_files = 1,
)

filegroup(
  name = 'mingw_all_files_win64',
  srcs = [
    '//tools/windows/win64:tool-wrappers',
    '@mingw_compiler_win64//:compiler_pieces',
  ],
)

filegroup(
  name = 'mingw_linker_files_win64',
  srcs = [
    '//tools/windows/win64:gcc',
    '//tools/windows/win64:ld',
    '//tools/windows/win64:ar',
    '@mingw_compiler_win64//:compiler_pieces',
  ],
)

filegroup(
  name = 'mingw_compiler_files_win64',
  srcs = [
    '//tools/windows/win64:gcc',
    '//tools/windows/win64:ld',
    '//tools/windows/win64:as',
    '@mingw_compiler_win64//:compiler_pieces',
  ],
)

cc_toolchain(
  name = 'cc-compiler-win64',
  all_files = ':mingw_all_files_win64',
  compiler_files = ':mingw_compiler_files_win64',
  cpu = 'win64',
  dwp_files = ':empty',
  dynamic_runtime_libs = [':empty'],
  linker_files = ':mingw_linker_files_win64',
  objcopy_files = '//tools/windows/win64:objcopy',
  static_runtime_libs = [':empty'],
  strip_files = '//tools/windows/win64:strip',
  supports_param_files = 1,
)

cc_toolchain(
    name = "cc-compiler-local",
    all_files = ":empty",
    compiler_files = ":empty",
    cpu = "local",
    dwp_files = ":empty",
    dynamic_runtime_libs = [":empty"],
    linker_files = ":empty",
    objcopy_files = ":empty",
    static_runtime_libs = [":empty"],
    strip_files = ":empty",
    supports_param_files = 1,
)

filegroup(
    name = "srcs",
    srcs = glob(["**"]),
)

config_setting(
    name = "win32_mode",
    values = { "cpu": "win32" }
)

config_setting(
    name = "win64_mode",
    values = { "cpu": "win64" }
)

py_binary(
    name = "zip4win",
    srcs = ["zip4win.py"],
)
