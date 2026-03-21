#![cfg(feature = "prost")]

use std::path::PathBuf;

use tempfile::tempdir;
use tinc_build::{Config, Mode};

fn proto_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/protos")
}

fn compile_proto(rel_path: &str) -> tinc_build::BuildError {
    let dir = tempdir().expect("tempdir");
    let protos = proto_dir();
    let proto_path = protos.join(rel_path);
    Config::new_with_out_dir(Mode::Prost, dir.path())
        .compile_protos(&[proto_path], &[protos])
        .expect_err("compile should fail")
}

macro_rules! snapshot_compile_error {
    ($($name:ident),+$(,)?) => {
        $(
            #[test]
            fn $name() {
                let err = compile_proto(stringify!($name.proto));
                insta::assert_snapshot!(err);
            }
        )*
    };
}

snapshot_compile_error![
    empty_package,
    client_streaming_with_http_endpoint,
    cel_message_level_parse_error,
    cel_message_level_resolve_error,
    cel_field_constraint_parse_error,
    cel_field_constraint_resolve_error,
];
