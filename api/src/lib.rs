#![no_std]
extern crate alloc;
mod call;
pub mod frame;
pub use call::Call;
pub use frame::Request;
pub use system_api::control::construction::{CANCEL, CreateId, INSPECT};
pub const CREATE: u8 = 1;
pub use interface::{BACK, CHANNELS, ENTRY, INTERFACE_ID, REGISTRY};
pub const DIR: &system_api::operator::Path = system_api::operator::Path::new("account/create");

#[mold::interface(id = "sqware.service.account.v1", metadata)]
mod interface {
    #[channels]
    pub enum Channel {
        #[channel(key = "create", legacy = "control-account", constant = ENTRY, publication = "create")]
        Create,
        #[channel(key = "reply", legacy = "control-account-back", constant = BACK)]
        Reply,
    }
}
