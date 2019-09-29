filegroup (
  name = "go_gen_protos",
  srcs = glob(["*_gw.go.proto"]),
)

proto_library (
  name = "go_gw_proto",
  srcs = [":go_gen_protos"],
  languages = ["go", "js"]
)
