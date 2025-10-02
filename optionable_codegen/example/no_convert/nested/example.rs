struct MemberNestedOpt {
    name: Option<String>,
    addresses: Option<<Vec<Address> as ::optionable::Optionable>::Optioned>,
}
#[automatically_derived]
impl ::optionable::Optionable for MemberNested {
    type Optioned = MemberNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for MemberNestedOpt {
    type Optioned = MemberNestedOpt;
}


struct AddressNestedOpt {
    street_name: Option<String>,
    number: Option<u8>,
}
#[automatically_derived]
impl ::optionable::Optionable for AddressNested {
    type Optioned = AddressNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for AddressNestedOpt {
    type Optioned = AddressNestedOpt;
}


enum AddressEnumNestedOpt {
    Unit,
    Plain(Option<String>),
    AddressExplicit { street: Option<String>, number: Option<u32> },
    AddressNested(Option<<Address as ::optionable::Optionable>::Optioned>),
}
#[automatically_derived]
impl ::optionable::Optionable for AddressEnumNested {
    type Optioned = AddressEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for AddressEnumNestedOpt {
    type Optioned = AddressEnumNestedOpt;
}
