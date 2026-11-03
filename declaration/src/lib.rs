#![no_std]
use unit::{Ending, ImageBuild, UnitFile};
pub static UNIT: UnitFile = UnitFile {
    name: "account",
    loader: unit::LoaderConfig::user("account"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "account",
            scope: unit::operator::Scope::Subtree,
            ops: system_api::operator::Ops::PART
                .union(system_api::operator::Ops::UNPART)
                .union(system_api::operator::Ops::BIND)
                .union(system_api::operator::Ops::UNBIND)
                .union(system_api::operator::Ops::FIND),
            permit: Some(system_api::operator::Permit::Bound),
            source_mark: None,
            bindings: &[unit::operator::Binding {
                name: "create",
                mark: account_api::ENTRY,
                view: env::Access::STORE,
                permit: system_api::operator::Permit::Bound,
            }],
        }],
    },
    identity: unit::Identity {
        aliases: &["anran"],
        grants: &[system_api::identity::Grant::Derive],
    },
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Bootstrap(unit::Bootstrap::Identity),
        ]),
        ready: unit::Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),

        construction: &[unit::Construction::unit_owned_by("shell", "login")],
        retirement: &[],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "account",
    manifest: "server/Cargo.toml",
    package: "account-server",
    bin: "account",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
