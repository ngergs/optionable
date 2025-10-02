#[derive(serde::Serialize, serde::Deserialize)]
struct MemberNestedOpt {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[automatically_derived]
impl ::optionable::OptionableConvert for MemberNested {
    fn into_optioned(self) -> MemberNestedOpt {
        MemberNestedOpt {
            name: Some(self.name),
            addresses: Some(
                <Vec<
                    Address,
                > as ::optionable::OptionableConvert>::into_optioned(self.addresses),
            ),
        }
    }
    fn try_from_optioned(
        value: MemberNestedOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["name"],
                })?,
            addresses: <Vec<
                Address,
            > as ::optionable::OptionableConvert>::try_from_optioned(
                value
                    .addresses
                    .ok_or(optionable::optionable::Error {
                        missing_fields: std::vec!["addresses"],
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: MemberNestedOpt,
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


#[derive(serde::Serialize, serde::Deserialize)]
struct AddressNestedOpt {
    #[serde(skip_serializing_if = "Option::is_none")]
    street_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[automatically_derived]
impl ::optionable::OptionableConvert for AddressNested {
    fn into_optioned(self) -> AddressNestedOpt {
        AddressNestedOpt {
            street_name: Some(self.street_name),
            number: Some(self.number),
        }
    }
    fn try_from_optioned(
        value: AddressNestedOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(Self {
            street_name: value
                .street_name
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["street_name"],
                })?,
            number: value
                .number
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["number"],
                })?,
        })
    }
    fn merge(
        &mut self,
        other: AddressNestedOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        if let Some(other_value) = other.street_name {
            self.street_name = other_value;
        }
        if let Some(other_value) = other.number {
            self.number = other_value;
        }
        Ok(())
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
enum AddressEnumNestedOpt {
    Unit,
    Plain(#[serde(skip_serializing_if = "Option::is_none")] Option<String>),
    AddressExplicit {
        #[serde(skip_serializing_if = "Option::is_none")]
        street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<u32>,
    },
    AddressNested(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<Address as ::optionable::Optionable>::Optioned>,
    ),
}
#[automatically_derived]
impl ::optionable::Optionable for AddressEnumNested {
    type Optioned = AddressEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for AddressEnumNestedOpt {
    type Optioned = AddressEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for AddressEnumNested {
    fn into_optioned(self) -> AddressEnumNestedOpt {
        match self {
            Self::Unit => AddressEnumNestedOpt::Unit,
            Self::Plain(self_0) => AddressEnumNestedOpt::Plain(Some(self_0)),
            Self::AddressExplicit { street: self_street, number: self_number } => {
                AddressEnumNestedOpt::AddressExplicit {
                    street: Some(self_street),
                    number: Some(self_number),
                }
            }
            Self::AddressNested(self_0) => {
                AddressEnumNestedOpt::AddressNested(
                    Some(
                        <Address as ::optionable::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
        }
    }
    fn try_from_optioned(
        other: AddressEnumNestedOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(
            match other {
                AddressEnumNestedOpt::Unit => Self::Unit,
                AddressEnumNestedOpt::Plain(other_0) => {
                    Self::Plain(
                        other_0
                            .ok_or(optionable::optionable::Error {
                                missing_fields: std::vec!["0"],
                            })?,
                    )
                }
                AddressEnumNestedOpt::AddressExplicit {
                    street: other_street,
                    number: other_number,
                } => {
                    Self::AddressExplicit {
                        street: other_street
                            .ok_or(optionable::optionable::Error {
                                missing_fields: std::vec!["street"],
                            })?,
                        number: other_number
                            .ok_or(optionable::optionable::Error {
                                missing_fields: std::vec!["number"],
                            })?,
                    }
                }
                AddressEnumNestedOpt::AddressNested(other_0) => {
                    Self::AddressNested(
                        <Address as ::optionable::OptionableConvert>::try_from_optioned(
                            other_0
                                .ok_or(optionable::optionable::Error {
                                    missing_fields: std::vec!["0"],
                                })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(
        &mut self,
        other: AddressEnumNestedOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        match other {
            AddressEnumNestedOpt::Unit => {
                if let Self::Unit = self {} else {
                    *self = Self::try_from_optioned(AddressEnumNestedOpt::Unit)?;
                }
            }
            AddressEnumNestedOpt::Plain(other_0) => {
                if let Self::Plain(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        AddressEnumNestedOpt::Plain(other_0),
                    )?;
                }
            }
            AddressEnumNestedOpt::AddressExplicit {
                street: other_street,
                number: other_number,
            } => {
                if let Self::AddressExplicit {
                    street: self_street,
                    number: self_number,
                } = self {
                    if let Some(other_value) = other_street {
                        *self_street = other_value;
                    }
                    if let Some(other_value) = other_number {
                        *self_number = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(AddressEnumNestedOpt::AddressExplicit {
                        street: other_street,
                        number: other_number,
                    })?;
                }
            }
            AddressEnumNestedOpt::AddressNested(other_0) => {
                if let Self::AddressNested(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <Address as ::optionable::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        AddressEnumNestedOpt::AddressNested(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
