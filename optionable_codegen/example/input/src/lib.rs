use crate::address::Address;

mod address;

struct Member {
    name: String,
    addresses: Vec<Address>,
}

#[cfg(test)]
mod test {
    use crate::address::Address;
    use crate::Member;

    struct MemberTest {
        name: String,
        addresses: Vec<Address>,
    }

    #[test]
    fn test_member() {
        let _ = Member {
            name: "test".to_string(),
            addresses: vec![Address::NumberOnly(123)],
        };
    }
}
