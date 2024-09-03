package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "routes",
    srcs = ["lib.rs"],
    deps = [
        ":plain",
        ":blog",
    ],
)

rust_library(
    name = "blog",
    srcs = ["blog.rs"],
    deps = [
        "//app:types",
        "//app:state",
        "@crate_index//:minijinja",
        "@crate_index//:axum",
        "@crate_index//:walkdir",
        "@crate_index//:serde",
        "@crate_index//:markdown",
    ]
)

rust_library(
    name = "plain",
    srcs = [
        "plain.rs"
    ],
    deps = [
        "//app:types",
        "//app:version",
        "//app:state",
        "@crate_index//:minijinja",
        "@crate_index//:axum",
    ]
)
