#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use account_api::Request;
    use env::PieToken;
    use env::wire::Span as _;
    use wire::message::Message;

    fn token(value: u64) -> PieToken {
        PieToken::from_bytes(&value.to_le_bytes()).unwrap()
    }

    #[test]
    fn request_keeps_its_original_64_byte_frame_and_raw_layout() {
        let request = Request {
            op: account_api::CREATE,
            id: system_api::control::construction::CreateId { value: 13 },
            account: String::from("ab"),
            back: token(0x0102_0304_0506_0708),
        };
        assert_eq!(Request::LEN, 64);
        assert_eq!(Request::EMPTY.len(), 64);

        let mut bytes = Request::EMPTY;
        let n = request.store(&mut bytes).unwrap();
        assert_eq!(n, 20);
        assert_eq!(
            &bytes[..n],
            &[
                1, 13, 0, 0, 0, 0, 0, 0, 0, 2, b'a', b'b', 8, 7, 6, 5, 4, 3, 2, 1,
            ]
        );

        let (decoded, exact) = Request::fetch(&bytes[..n]).unwrap();
        assert!(exact);
        assert_eq!(decoded.op, account_api::CREATE);
        assert_eq!(decoded.id.value, 13);
        assert_eq!(decoded.account, "ab");
        assert_eq!(decoded.back, request.back);
        assert_eq!(account_api::Call::back(&(decoded, exact)), request.back);
        assert_eq!(account_api::Call::BACK, account_api::BACK);
    }

    #[test]
    fn prefix_decoding_is_not_exact_and_take_rejects_the_tail() {
        let request = Request {
            op: account_api::CREATE,
            id: system_api::control::construction::CreateId { value: 1 },
            account: String::from("anran"),
            back: token(0x1234),
        };
        let mut bytes = [0; Request::LEN + 1];
        let n = request.store_at(&mut bytes, 0).unwrap();
        assert!(Request::fetch(&bytes[..n]).unwrap().1);
        let taken = Request::take(&bytes[..n]).unwrap();
        assert_eq!(taken.account, "anran");
        assert_eq!(taken.back, token(0x1234));
        let (prefix, exact) = Request::fetch(&bytes[..n + 1]).unwrap();
        assert!(!exact);
        assert_eq!(prefix.back, token(0x1234));
        assert!(Request::take(&bytes[..n + 1]).is_none());
    }

    #[test]
    fn account_names_stop_at_31_bytes_but_the_frame_holds_the_32nd_until_take() {
        let mut bytes = [0; Request::LEN + 1];
        for len in [31, 32] {
            let request = Request {
                op: account_api::CREATE,
                id: system_api::control::construction::CreateId { value: 1 },
                account: "a".repeat(len),
                back: token(1),
            };
            let n = request.store_at(&mut bytes, 0).unwrap();
            assert_eq!(n, 18 + len);
            assert_eq!(Request::take(&bytes[..n]).is_some(), len == 31);
            assert!(Request::fetch(&bytes[..n]).is_some());
        }

        let oversized = Request {
            op: account_api::CREATE,
            id: system_api::control::construction::CreateId { value: 1 },
            account: "a".repeat(47),
            back: token(1),
        };
        let mut frame = Request::EMPTY;
        assert!(oversized.store(&mut frame).is_none());
    }

    #[test]
    fn operation_and_status_numbers_remain_the_original_wire_values() {
        use system_api::control::construction as construction;
        assert_eq!(account_api::CREATE, construction::BUILD);
        assert_eq!(account_api::CREATE, 1);
        assert_eq!(account_api::INSPECT, 2);
        assert_eq!(account_api::CANCEL, 3);
        assert_eq!(construction::UNSEEN, 0);
        assert_eq!(construction::PENDING, 1);
        assert_eq!(construction::COMPLETED, 2);
        assert_eq!(construction::FAILED, 3);
        assert_eq!(construction::CANCELLED, 4);
        assert_eq!(construction::CLOSED, 5);
        assert_eq!(system_api::control::frame::OK, 0);
        assert_eq!(account_api::INTERFACE_ID, "sqware.service.account.v1");
    }
}
