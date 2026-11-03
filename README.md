# Account Unit

Standalone Account Unit repository owning the Account API, client, server,
declaration and Unit-owned host tests. It was extracted from Source
`293c15fc97a9d9a4295810d10fe804e4cdec63c4`.

The root workspace builds `api`, `client`, `declaration` and `server`; host
tests under `tests` remain separate workspaces. The declaration `BUILD` targets
this repository's `server/Cargo.toml`, package `account-server`, binary
`account`.

The Session Cluster consumes this Unit as a fixed public Git submodule. The
Cluster owns the explicit member/build interface and pinned Unit revision;
combined composition and Deployment selection remain Source/Vanllia. Root will
publish verified provenance, maps and locks for this standalone Unit.
