#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) enum AddressOpt {
    NumberOnly(#[serde(skip_serializing_if = "Option::is_none")] Option<u64>),
    Address(#[serde(skip_serializing_if = "Option::is_none")] Option<String>),
}
#[automatically_derived]
impl ::optionable::Optionable for address::Address {
    type Optioned = AddressOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for AddressOpt {
    type Optioned = AddressOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for address::Address {
    fn into_optioned(self) -> AddressOpt {
        match self {
            Self::NumberOnly(self_0) => AddressOpt::NumberOnly(Some(self_0)),
            Self::Address(self_0) => AddressOpt::Address(Some(self_0)),
        }
    }
    fn try_from_optioned(other: AddressOpt) -> Result<Self, ::optionable::Error> {
        Ok(
            match other {
                AddressOpt::NumberOnly(other_0) => {
                    Self::NumberOnly(
                        other_0
                            .ok_or(::optionable::Error {
                                missing_field: "0",
                            })?,
                    )
                }
                AddressOpt::Address(other_0) => {
                    Self::Address(
                        other_0
                            .ok_or(::optionable::Error {
                                missing_field: "0",
                            })?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: AddressOpt) -> Result<(), ::optionable::Error> {
        match other {
            AddressOpt::NumberOnly(other_0) => {
                if let Self::NumberOnly(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(AddressOpt::NumberOnly(other_0))?;
                }
            }
            AddressOpt::Address(other_0) => {
                if let Self::Address(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(AddressOpt::Address(other_0))?;
                }
            }
        }
        Ok(())
    }
}
