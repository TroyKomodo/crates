# Instructions for coding agents

This repository is a **Rust workspace** (edition **2024**, resolver `"2"`). Licensed **MIT OR Apache-2.0**. See the root [README.md](./README.md) for the crate list and short descriptions.

## Project policy

- **Pull requests** are expected to be opened by humans, not by automated agents ([README](./README.md)).

## How to validate changes

CI treats warnings as errors (`RUSTFLAGS=-Dwarnings`, `RUSTDOCFLAGS=-Dwarnings`). Prefer matching that locally.

### Run commands through the Nix dev shell

**Do not** invoke `cargo`, `just`, `rustfmt`, etc. directly on the host unless Nix is unavailable. From the repository root, wrap them with `nix develop --command` so you use the same toolchain and helpers as [flake.nix](./flake.nix) / CI (`just`, `dprint`, `buf`, `cargo-nextest`, `cargo-llvm-cov`, etc.).

Examples:

| Check           | Command                                                                                                            |
| --------------- | ------------------------------------------------------------------------------------------------------------------ |
| Format          | `nix develop --command just fmt`                                                                                   |
| Tests + cov     | `nix develop --command just test`                                                                                  |
| Docs            | `nix develop --command just doc`                                                                                   |
| README sync     | `nix develop --command just sync-readme-test`                                                                      |
| Clippy (CI bar) | `nix develop --command env RUSTFLAGS=-Dwarnings RUSTDOCFLAGS=-Dwarnings cargo clippy --all-targets --all-features` |

One-off `cargo` invocations (tests for one package, `cargo check`, etc.) should use the same wrapper, for example:

```bash
nix develop --command cargo test -p tinc-build
```

### If Nix is not available

Fallback (may diverge from CI): install the needed tools locally, set `RUSTFLAGS` / `RUSTDOCFLAGS` as above, then run `cargo fmt`, `cargo clippy`, `cargo test` / `cargo nextest` as appropriate.

## Conventions

- **Crate deny policy**: Before editing a crate, read the inner attributes at the top of its `src/lib.rs` (for example `#![deny(missing_docs)]`, `#![deny(unsafe_code)]`, `#![deny(unreachable_pub)]`, `#![deny(clippy::…)]`). Do not introduce code that violates those crate-level rules. If a crate root is `main.rs` instead, check there for the same pattern.
- **Scope**: Change only what the task needs; avoid drive-by refactors and unrelated formatting.
- **Style**: Follow the surrounding module (naming, error handling, imports). Prefer extending existing helpers over duplicating logic.
- **Comments**: Prefer self-explanatory code; avoid obvious comments, `// region` blocks, and noisy doc comments on trivial items.
- **Workspace**: Crate manifests live under top-level directories listed in [Cargo.toml](./Cargo.toml) `members`.

When unsure about behavior, read the crate’s `src/` and existing tests before editing.
