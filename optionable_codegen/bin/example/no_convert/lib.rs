mod address;
struct MemberOpt {
    name: Option<String>,
    addresses: Option<<Vec<Address> as ::optionable::Optionable>::Optioned>,
}
#[automatically_derived]
impl ::optionable::Optionable for Member {
    type Optioned = MemberOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for MemberOpt {
    type Optioned = MemberOpt;
}
#[cfg(test)]
mod test {
    struct MemberTestOpt {
        name: Option<String>,
        addresses: Option<<Vec<Address> as ::optionable::Optionable>::Optioned>,
    }
    #[automatically_derived]
    impl ::optionable::Optionable for MemberTest {
        type Optioned = MemberTestOpt;
    }
    #[automatically_derived]
    impl ::optionable::Optionable for MemberTestOpt {
        type Optioned = MemberTestOpt;
    }
}
