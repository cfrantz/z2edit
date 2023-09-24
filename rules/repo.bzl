load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# A local repository rule which refers to an archive file.
def _local_archive_impl(rctx):
    if rctx.attr.build_file and rctx.attr.build_file_content:
        fail("Use only one of build_file and build_file_content.")

    rctx.extract(
        archive = rctx.attr.path,
        stripPrefix = rctx.attr.strip_prefix,
    )
    if rctx.attr.build_file:
        rctx.symlink(rctx.attr.build_file, "BUILD.bazel")
    elif rctx.attr.build_file_content:
        rctx.file("BUILD.bazel", rctx.attr.build_file_content)

local_archive = repository_rule(
    implementation = _local_archive_impl,
    attrs = {
        "path": attr.string(doc = "Local path to the archive", mandatory = True),
        "strip_prefix": attr.string(doc = "Strip path prefixes when unarchiving"),
        "build_file": attr.label(doc = "A file to use as a BUILD file for this repository", allow_single_file = True),
        "build_file_content": attr.string(doc = "The content for the BUILD file for this repository"),
    },
)

def _is_archive(filename):
    return (
        filename.endswith(".tar") or
        filename.endswith(".tar.gz") or
        filename.endswith(".tar.bz") or
        filename.endswith(".tar.bz2") or
        filename.endswith(".tar.xz") or
        filename.endswith(".zip")
    )

# A local repository rule which selects either local_repository or
# new_local_repository depending on the supplied arguments.
def local_repository(
        name,
        path,
        build_file = None,
        build_file_content = None,
        workspace_file = None,
        workspace_file_content = None):
    if build_file or build_file_content or workspace_file or workspace_file_content:
        native.new_local_repository(
            name = name,
            path = path,
            build_file = build_file,
            build_file_content = build_file_content,
        )
    else:
        native.local_repository(
            name = name,
            path = path,
        )

def http_archive_or_local(local = None, **kwargs):
    if local:
        build_file = kwargs.get("build_file")
        build_file_content = kwargs.get("build_file_content")
        if _is_archive(local):
            local_archive(
                name = kwargs.get("name"),
                path = local,
                strip_prefix = kwargs.get("strip_prefix"),
                build_file = build_file,
                build_file_content = build_file_content,
            )
        else:
            local_repository(
                name = kwargs.get("name"),
                path = local,
                build_file = build_file,
                build_file_content = build_file_content,
            )
    else:
        http_archive(**kwargs)
