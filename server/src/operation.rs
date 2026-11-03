use alloc::string::String;
use env::TaskId;
use system_api::identity::Subject;

#[derive(Clone)]
pub(super) struct Operation {
    pub(super) caller: TaskId,
    pub(super) id: u64,
    pub(super) account: String,
    pub(super) owner: TaskId,
    pub(super) subject: Subject,
    pub(super) host: TaskId,
    pub(super) constructor_id: system_api::control::construction::CreateId,
    pub(super) cancel_requested: bool,
    pub(super) terminal: bool,
}
