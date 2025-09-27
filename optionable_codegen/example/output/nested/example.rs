#[automatically_derived]
struct TestStructNestedOpt {
    street_nested: Option<String>,
    number_nested: Option<i32>,
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructNested {
    type Optioned = TestStructNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructNestedOpt {
    type Optioned = TestStructNestedOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for TestStructNested {
    fn into_optioned(self) -> TestStructNestedOpt {
        TestStructNestedOpt {
            street_nested: Some(self.street_nested),
            number_nested: Some(self.number_nested),
        }
    }
    fn try_from_optioned(
        value: TestStructNestedOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(Self {
            street_nested: value
                .street_nested
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["street_nested"],
                })?,
            number_nested: value
                .number_nested
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["number_nested"],
                })?,
        })
    }
    fn merge(
        &mut self,
        other: TestStructNestedOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        if let Some(other_value) = other.street_nested {
            self.street_nested = other_value;
        }
        if let Some(other_value) = other.number_nested {
            self.number_nested = other_value;
        }
        Ok(())
    }
}


#[automatically_derived]
enum TestEnumNestedOpt {
    NumberNested(Option<i32>),
    AddressNested(Option<<TestStructNested as ::optionable::Optionable>::Optioned>),
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumNested {
    type Optioned = TestEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumNestedOpt {
    type Optioned = TestEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for TestEnumNested {
    fn into_optioned(self) -> TestEnumNestedOpt {
        match self {
            Self::NumberNested(self_0) => TestEnumNestedOpt::NumberNested(Some(self_0)),
            Self::AddressNested(self_0) => {
                TestEnumNestedOpt::AddressNested(
                    Some(
                        <TestStructNested as ::optionable::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
        }
    }
    fn try_from_optioned(
        other: TestEnumNestedOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(
            match other {
                TestEnumNestedOpt::NumberNested(other_0) => {
                    Self::NumberNested(
                        other_0
                            .ok_or(optionable::optionable::Error {
                                missing_fields: std::vec!["0"],
                            })?,
                    )
                }
                TestEnumNestedOpt::AddressNested(other_0) => {
                    Self::AddressNested(
                        <TestStructNested as ::optionable::OptionableConvert>::try_from_optioned(
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
        other: TestEnumNestedOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        match other {
            TestEnumNestedOpt::NumberNested(other_0) => {
                if let Self::NumberNested(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        TestEnumNestedOpt::NumberNested(other_0),
                    )?;
                }
            }
            TestEnumNestedOpt::AddressNested(other_0) => {
                if let Self::AddressNested(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <TestStructNested as ::optionable::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        TestEnumNestedOpt::AddressNested(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
