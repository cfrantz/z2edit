load("@google_protobuf//:protobuf.bzl", "cc_proto_library")

def proto_library(*args, **kwargs):
	if kwargs.get("default_runtime", None) == None:
		kwargs["default_runtime"] = "@google_protobuf//:protobuf"
	if kwargs.get("protoc", None) == None:
		kwargs["protoc"] = "@google_protobuf//:protoc"
	cc_proto_library(*args, **kwargs)
