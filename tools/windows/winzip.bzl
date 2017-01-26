
def winzip(name, srcs, zipfile):
    arch = select({
        '//tools/windows:win32_mode': 'win32',
        '//tools/windows:win64_mode': 'win64',
    })

    native.genrule(
        name = name,
        srcs = srcs,
        outs = [ zipfile ],
        cmd = ('$(location //tools/windows:zip4win)'
               + ' --out $@'
               + ' --mxe external/mingw_compiler_' + arch
               + ' --target ' + arch
               + ' $(SRCS)'),
        tools = select({
            '//tools/windows:win32_mode': [
                '//tools/windows/win32:objdump',
                '@mingw_compiler_win32//:compiler_pieces',
            ],
            '//tools/windows:win64_mode': [
                '//tools/windows/win64:objdump',
                '@mingw_compiler_win64//:compiler_pieces',
            ],
        }) + [
            '//tools/windows:zip4win',
        ],
    )
