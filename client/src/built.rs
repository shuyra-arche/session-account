use env::{TaskId, TeamId};

#[derive(Clone, Copy, Debug)]
pub struct Built {
    pub task: TaskId,
    pub team: TeamId,
    pub startup: system_api::loader::StartupMetadata,
}
