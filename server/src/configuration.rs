use alloc::vec::Vec;
use env::TaskId;

use crate::operation::Operation;

pub(super) struct Configuration {
    pub(super) supervisor: TaskId,
    pub(super) next_create_id: u64,
    pub(super) operations: Vec<Operation>,
    pub(super) watermarks: Vec<(TaskId, u64)>,
}
impl Configuration {
    pub(super) fn reserve(&mut self, caller: TaskId, id: u64) -> Result<(), system_api::control::Fail> {
        use system_api::control::Fail;
        if id == 0 {
            return Err(Fail::Bad);
        }
        if let Some((_, high)) = self.watermarks.iter().find(|(task, _)| *task == caller) {
            if id <= *high {
                return Err(Fail::Conflict);
            }
        } else {
            if self.watermarks.len() >= 64 {
                return Err(Fail::Full);
            }
            self.watermarks.try_reserve(1).map_err(|_| Fail::Full)?;
        }
        if self.operations.len() >= 256 {
            if let Some(at) = self.operations.iter().position(|record| record.terminal) {
                self.operations.swap_remove(at);
            } else {
                return Err(Fail::Full);
            }
        } else {
            self.operations.try_reserve(1).map_err(|_| Fail::Full)?;
        }
        if let Some((_, high)) = self.watermarks.iter_mut().find(|(task, _)| *task == caller) {
            *high = id;
        } else {
            self.watermarks.push((caller, id));
        }
        Ok(())
    }
    pub(super) fn allocate(
        &mut self,
    ) -> Result<system_api::control::construction::CreateId, system_api::control::Fail> {
        let value = self.next_create_id;
        self.next_create_id = value
            .checked_add(1)
            .ok_or(system_api::control::Fail::Full)?;
        Ok(system_api::control::construction::CreateId { value })
    }
}
