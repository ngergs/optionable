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
#[automatically_derived]
impl ::optionable::OptionableConvert for Member {
    fn into_optioned(self) -> MemberOpt {
        MemberOpt {
            name: Some(self.name),
            addresses: Some(
                <Vec<
                    Address,
                > as ::optionable::OptionableConvert>::into_optioned(self.addresses),
            ),
        }
    }
    fn try_from_optioned(
        value: MemberOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(::optionable::optionable::Error {
                    missing_fields: std::vec!["name"],
                })?,
            addresses: <Vec<
                Address,
            > as ::optionable::OptionableConvert>::try_from_optioned(
                value
                    .addresses
                    .ok_or(::optionable::optionable::Error {
                        missing_fields: std::vec!["addresses"],
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: MemberOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.addresses {
            <Vec<
                Address,
            > as ::optionable::OptionableConvert>::merge(
                &mut self.addresses,
                other_value,
            )?;
        }
        Ok(())
    }
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
    #[automatically_derived]
    impl ::optionable::OptionableConvert for MemberTest {
        fn into_optioned(self) -> MemberTestOpt {
            MemberTestOpt {
                name: Some(self.name),
                addresses: Some(
                    <Vec<
                        Address,
                    > as ::optionable::OptionableConvert>::into_optioned(self.addresses),
                ),
            }
        }
        fn try_from_optioned(
            value: MemberTestOpt,
        ) -> Result<Self, ::optionable::optionable::Error> {
            Ok(Self {
                name: value
                    .name
                    .ok_or(::optionable::optionable::Error {
                        missing_fields: std::vec!["name"],
                    })?,
                addresses: <Vec<
                    Address,
                > as ::optionable::OptionableConvert>::try_from_optioned(
                    value
                        .addresses
                        .ok_or(::optionable::optionable::Error {
                            missing_fields: std::vec!["addresses"],
                        })?,
                )?,
            })
        }
        fn merge(
            &mut self,
            other: MemberTestOpt,
        ) -> Result<(), ::optionable::optionable::Error> {
            if let Some(other_value) = other.name {
                self.name = other_value;
            }
            if let Some(other_value) = other.addresses {
                <Vec<
                    Address,
                > as ::optionable::OptionableConvert>::merge(
                    &mut self.addresses,
                    other_value,
                )?;
            }
            Ok(())
        }
    }
}
