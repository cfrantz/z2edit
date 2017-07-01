# A simplistic packaing rule for windows binaries.
#
def _pkg_winzip(ctx):
    args = [
        "--mxe=" + ctx.files.mxe[0].path,
        "--out=" + ctx.outputs.out.path,
    ]
    if ctx.attr.skip_dlls:
        args += ["--skip_dlls=" + ','.join(ctx.attr.skip_dlls)]

    args += [f.path for f in ctx.files.files]
    ctx.action(
        executable = ctx.executable.zip4win,
        arguments = args,
        inputs = ctx.files.files + ctx.files.mxe,
        outputs = [ctx.outputs.out],
        progress_message="Packaging files into " + ctx.outputs.out.basename,
        mnemonic="WinZip"
    )

pkg_winzip = rule(
    implementation = _pkg_winzip,
    attrs = {
        "files": attr.label_list(allow_files=True),
        "skip_dlls": attr.string_list(),
        "mxe": attr.label(
            default=Label("//tools:mxe"),
            allow_files=True),
        "zip4win": attr.label(
            default=Label("//tools/windows:zip4win"),
            cfg="host",
            executable=True,
            allow_files=True)
    },
    outputs = {
        "out":  "%{name}.zip",
    },
)
