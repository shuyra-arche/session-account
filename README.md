# Account

Account provides the API, client and resident server for account-managed Shell
construction in Shuyra Vanllia.

## Layout and policy

- `api/` and `client/`: wire protocol and typed client.
- `server/`: server library and the `account` executable.
- `unit.toml`: namespace, identity, readiness and construction policy.
- `declaration/`: retained Rust Unit and image-build compatibility declarations.
- `tests/api` and `tests/configuration`: independent host test workspaces.

Account packages the requested Shell template as a dependency. Its construction
policy associates the Shell owner with Login and limits arguments and stack.
System completes Operator and Identity bootstrap before starting Account.
Session selects this provider; Vanllia packages and runs it.

## Build and test

Install Rust through rustup; `rust-toolchain.toml` selects `nightly-2026-09-04`
and the RISC-V target. From the root, with network access for the first fetch:

```sh
cargo build -p account-server --bin account --target riscv64gc-unknown-none-elf --release --locked
cargo test --manifest-path tests/api/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/configuration/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

Host tests cover wire and state behavior; validate actual create/inspect/cancel
handover and reclamation in the selected Vanllia deployment.
Source attribution is retained in [PROVENANCE.md](PROVENANCE.md).

Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
