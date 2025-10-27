pub(crate) enum AddressOpt {
    NumberOnly(Option<u64>),
    Address(Option<String>),
}
#[automatically_derived]
impl ::optionable::Optionable for address::Address {
    type Optioned = AddressOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for AddressOpt {
    type Optioned = AddressOpt;
}
