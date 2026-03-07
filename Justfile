default:
    @just --list

fmt:
    dprint fmt
    buf format -w --disable-symlinks --debug
    just --unstable --fmt
    shfmt -w .
    nix fmt .

lint:
    cargo clippy --fix --allow-dirty --allow-staged

deny:
    cargo deny check

alias t := test

test:
    #!/usr/bin/env bash
    set -eo pipefail
    export RUSTC_BOOTSTRAP=1
    ci="${CI:-0}"
    [[ "$ci" == "0" ]] && export INSTA_FORCE_PASS=1

    test_exit_code=0
    doc_exit_code=0

    cargo llvm-cov clean
    cargo llvm-cov --no-report --include-build-script nextest || test_exit_code=$?
    cargo llvm-cov --no-report --include-build-script --doc || doc_exit_code=$?
    cargo llvm-cov report --doctests --lcov --include-build-script --output-path lcov.info

    if [[ "$ci" == "0" ]]; then
        cargo insta review
    fi

    if [[ "$test_exit_code" != "0" || "$doc_exit_code" != "0" ]]; then
        exit 1
    fi

doc:
    cargo doc --all-features --no-deps

doc-serve: doc
    miniserve target/doc

sync-readme:
    cargo run -p cargo-sync-readme2 -- workspace --target-dir target/sync-readme sync

sync-readme-test:
    cargo run -p cargo-sync-readme2 -- workspace --target-dir target/sync-readme test

clean:
    cargo clean