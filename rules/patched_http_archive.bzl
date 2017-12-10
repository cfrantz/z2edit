def _new_patched_http_archive_impl(rctx):
  # Download the archive and extract it
  rctx.download_and_extract(
      url=rctx.attr.urls,
      output=rctx.path(""),
      stripPrefix=rctx.attr.strip_prefix,
      type=rctx.attr.type,
      sha256=rctx.attr.sha256)

  # Now patch the repository
  patch_file = str(rctx.path(rctx.attr.patch).realpath)
  result = rctx.execute(["bash", "-c", "pwd && patch -p0 < " + patch_file])
  if result.return_code != 0:
    fail("Failed to patch (%s): %s" % (result.return_code, result.stdout))

  # And finally add the build file
  if hasattr(rctx.attr, 'build_file'):
    rctx.symlink(rctx.attr.build_file, "BUILD.bazel")

patched_http_archive = repository_rule(
    implementation=_new_patched_http_archive_impl,
    attrs={
        "urls": attr.string_list(mandatory=True),
        "patch": attr.label(mandatory=True),
        "sha256": attr.string(mandatory=True),
        "strip_prefix": attr.string(mandatory=False, default=""),
        "type": attr.string(mandatory=False, default=""),
    })

new_patched_http_archive = repository_rule(
    implementation=_new_patched_http_archive_impl,
    attrs={
        "urls": attr.string_list(mandatory=True),
        "patch": attr.label(mandatory=True),
        "sha256": attr.string(mandatory=True),
        "strip_prefix": attr.string(mandatory=False, default=""),
        "type": attr.string(mandatory=False, default=""),
        "build_file": attr.label(mandatory=True),
    })
