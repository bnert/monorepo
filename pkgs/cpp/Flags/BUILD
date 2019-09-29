cc_library (
  name = "Flags",
  srcs = ["Flags.cpp"],
  hdrs = ["Flags.h"],
)

cc_test (
  name = "FlagsTest",
  srcs = glob(["*.test.cpp"]),
  hdrs = glob(["*.test.h", "*.test.hpp"]),
  flags = "-t_two 1700 -t_four -t_six https://changed.url",
  deps = [
    ":Flags",
    "///third_party/cc/gtest//:gtest_main"
  ],
)
