package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "state",
    srcs = [
        "state.rs"
    ],
    deps = [
        "@crate_index//:minijinja"
    ]
)

rust_library(
    name = "types",
    srcs = ["types.rs"],
    deps = [
        "@crate_index//:struct_iterable",
        "@crate_index//:serde",
    ]
)

rust_library(
    name = "version",
    srcs = ["version.rs"]
)
