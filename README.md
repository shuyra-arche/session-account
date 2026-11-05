# Account Unit

Standalone Account Unit repository owning the Account API, client, server,
declaration and Unit-owned host tests. It was extracted from Source
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`.

The root workspace builds `api`, `client`, `declaration` and `server`; host
tests under `tests` remain separate workspaces. The declaration `BUILD` targets
this repository's `server/Cargo.toml`, package `account-server`, binary
`account`.

The root `unit.toml` declares the runtime Unit for the dynamic Vanllia build
path. It preserves the Account namespace, identity and construction policy and
requires the Shell declaration to be packaged for runtime construction. System
finishes its Operator and Identity bootstrap before starting Account. The Rust
declaration remains as a compatibility input for consumers awaiting migration.

The Session repository groups Units through fixed public Git submodules.
The host `vanllia` discovers Unit TOML declarations in the selected sources and
constructs each Deployment in memory. It packages the necessary declarations,
images and resources into initrd; System creates instances according to their
runtime declarations.
