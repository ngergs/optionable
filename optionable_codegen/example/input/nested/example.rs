struct MemberNested {
    name: String,
    addresses: Vec<Address>,
}

struct AddressNested {
    street_name: String,
    number: u8,
}

enum AddressEnumNested {
    Unit,
    Plain(String),
    AddressExplicit { street: String, number: u32 },
    AddressNested(Address),
}
