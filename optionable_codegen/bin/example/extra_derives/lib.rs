pub mod address;
#[derive(serde::Deserialize, serde::Serialize)]
struct MemberOpt {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
                ::optionable::OptionableConvert::into_optioned(self.addresses),
            ),
        }
    }
    fn try_from_optioned(value: MemberOpt) -> Result<Self, ::optionable::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(::optionable::Error {
                    missing_field: "name",
                })?,
            addresses: ::optionable::OptionableConvert::try_from_optioned(
                value
                    .addresses
                    .ok_or(::optionable::Error {
                        missing_field: "addresses",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: MemberOpt) -> Result<(), ::optionable::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.addresses {
            ::optionable::OptionableConvert::merge(&mut self.addresses, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl ::optionable::OptionedConvert<Member> for MemberOpt {
    fn from_optionable(value: Member) -> Self {
        ::optionable::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(self) -> Result<Member, ::optionable::Error> {
        ::optionable::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(self, other: &mut Member) -> Result<(), ::optionable::Error> {
        ::optionable::OptionableConvert::merge(other, self)
    }
}
#[cfg(test)]
mod test {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct MemberTestOpt {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
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
                    ::optionable::OptionableConvert::into_optioned(self.addresses),
                ),
            }
        }
        fn try_from_optioned(value: MemberTestOpt) -> Result<Self, ::optionable::Error> {
            Ok(Self {
                name: value
                    .name
                    .ok_or(::optionable::Error {
                        missing_field: "name",
                    })?,
                addresses: ::optionable::OptionableConvert::try_from_optioned(
                    value
                        .addresses
                        .ok_or(::optionable::Error {
                            missing_field: "addresses",
                        })?,
                )?,
            })
        }
        fn merge(&mut self, other: MemberTestOpt) -> Result<(), ::optionable::Error> {
            if let Some(other_value) = other.name {
                self.name = other_value;
            }
            if let Some(other_value) = other.addresses {
                ::optionable::OptionableConvert::merge(
                    &mut self.addresses,
                    other_value,
                )?;
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::optionable::OptionedConvert<MemberTest> for MemberTestOpt {
        fn from_optionable(value: MemberTest) -> Self {
            ::optionable::OptionableConvert::into_optioned(value)
        }
        fn try_into_optionable(self) -> Result<MemberTest, ::optionable::Error> {
            ::optionable::OptionableConvert::try_from_optioned(self)
        }
        fn merge_into(self, other: &mut MemberTest) -> Result<(), ::optionable::Error> {
            ::optionable::OptionableConvert::merge(other, self)
        }
    }
}
