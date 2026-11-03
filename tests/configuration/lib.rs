#![allow(dead_code)]
extern crate alloc;

#[path = "../../server/src/configuration.rs"]
mod configuration;
#[path = "../../server/src/operation.rs"]
mod operation;

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use env::TaskId;
    use system_api::control::Fail;
    use system_api::control::construction::CreateId;
    use system_api::identity::{PrincipalId, Subject};

    use crate::configuration::Configuration;
    use crate::operation::Operation;

    fn subject() -> Subject {
        Subject::new(PrincipalId::new(TaskId::new(1), 1), &[]).unwrap()
    }

    fn operation(caller: usize, id: u64, terminal: bool) -> Operation {
        Operation {
            caller: TaskId::new(caller),
            id,
            account: String::from("anran"),
            owner: TaskId::new(2),
            subject: subject(),
            host: TaskId::new(3),
            constructor_id: CreateId { value: id },
            cancel_requested: false,
            terminal,
        }
    }

    fn configuration() -> Configuration {
        Configuration {
            supervisor: TaskId::new(1),
            next_create_id: 1,
            operations: Vec::new(),
            watermarks: Vec::new(),
        }
    }

    #[test]
    fn watermarks_stop_at_64_callers_and_leave_the_65th_untouched() {
        let mut config = configuration();
        for caller in 1..=64 {
            assert!(config.reserve(TaskId::new(caller), 1).is_ok());
            assert_eq!(config.watermarks.len(), caller as usize);
        }
        assert_eq!(
            config.reserve(TaskId::new(65), 1),
            Err(Fail::Full)
        );
        assert_eq!(config.watermarks.len(), 64);
        assert!(!config
            .watermarks
            .iter()
            .any(|(caller, _)| *caller == TaskId::new(65)));
    }

    #[test]
    fn record_capacity_requires_one_terminal_entry_and_rejects_256_live_records() {
        let mut config = configuration();
        for n in 1..=256 {
            config.operations.push(operation(1, n, false));
        }
        assert_eq!(
            config.reserve(TaskId::new(9), 500),
            Err(Fail::Full)
        );
        assert_eq!(config.operations.len(), 256);
        assert!(config.watermarks.is_empty());

        config.operations[123].terminal = true;
        assert!(config.reserve(TaskId::new(9), 500).is_ok());
        assert_eq!(config.operations.len(), 255);
        assert!(!config.operations.iter().any(|record| record.id == 124));
        assert_eq!(config.watermarks, vec![(TaskId::new(9), 500)]);
    }

    #[test]
    fn replay_and_zero_ids_do_not_advance_watermarks() {
        let mut config = configuration();
        assert_eq!(config.reserve(TaskId::new(7), 0), Err(Fail::Bad));
        assert!(config.watermarks.is_empty());

        assert!(config.reserve(TaskId::new(7), 10).is_ok());
        assert_eq!(config.reserve(TaskId::new(7), 10), Err(Fail::Conflict));
        assert_eq!(config.reserve(TaskId::new(7), 9), Err(Fail::Conflict));
        assert_eq!(config.watermarks, vec![(TaskId::new(7), 10)]);

        assert!(config.reserve(TaskId::new(7), 11).is_ok());
        assert_eq!(config.watermarks, vec![(TaskId::new(7), 11)]);
    }

    #[test]
    fn create_id_allocation_is_checked_and_does_not_wrap() {
        let mut config = configuration();
        config.next_create_id = u64::MAX;
        assert_eq!(config.allocate(), Err(Fail::Full));
        assert_eq!(config.next_create_id, u64::MAX);
        assert_eq!(config.allocate(), Err(Fail::Full));

        config.next_create_id = 41;
        assert_eq!(config.allocate().unwrap().value, 41);
        assert_eq!(config.next_create_id, 42);
    }
}
