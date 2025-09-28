#[automatically_derived]
#[derive(serde::Serialize, serde::Deserialize)]
struct TestStructOpt {
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
}
#[automatically_derived]
impl ::optionable::Optionable for TestStruct {
    type Optioned = TestStructOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructOpt {
    type Optioned = TestStructOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for TestStruct {
    fn into_optioned(self) -> TestStructOpt {
        TestStructOpt {
            street: Some(self.street),
            number: Some(self.number),
        }
    }
    fn try_from_optioned(
        value: TestStructOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(Self {
            street: value
                .street
                .ok_or(optionable::optionable::Error {
                    missing_fields: std::vec!["street"],
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
        other: TestStructOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        if let Some(other_value) = other.street {
            self.street = other_value;
        }
        if let Some(other_value) = other.number {
            self.number = other_value;
        }
        Ok(())
    }
}


#[automatically_derived]
#[derive(serde::Serialize, serde::Deserialize)]
enum TestEnumOpt {
    Number(#[serde(skip_serializing_if = "Option::is_none")] Option<i32>),
    Address(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<TestStruct as ::optionable::Optionable>::Optioned>,
    ),
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnum {
    type Optioned = TestEnumOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumOpt {
    type Optioned = TestEnumOpt;
}
#[automatically_derived]
impl ::optionable::OptionableConvert for TestEnum {
    fn into_optioned(self) -> TestEnumOpt {
        match self {
            Self::Number(self_0) => TestEnumOpt::Number(Some(self_0)),
            Self::Address(self_0) => {
                TestEnumOpt::Address(
                    Some(
                        <TestStruct as ::optionable::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
        }
    }
    fn try_from_optioned(
        other: TestEnumOpt,
    ) -> Result<Self, ::optionable::optionable::Error> {
        Ok(
            match other {
                TestEnumOpt::Number(other_0) => {
                    Self::Number(
                        other_0
                            .ok_or(optionable::optionable::Error {
                                missing_fields: std::vec!["0"],
                            })?,
                    )
                }
                TestEnumOpt::Address(other_0) => {
                    Self::Address(
                        <TestStruct as ::optionable::OptionableConvert>::try_from_optioned(
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
        other: TestEnumOpt,
    ) -> Result<(), ::optionable::optionable::Error> {
        match other {
            TestEnumOpt::Number(other_0) => {
                if let Self::Number(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(TestEnumOpt::Number(other_0))?;
                }
            }
            TestEnumOpt::Address(other_0) => {
                if let Self::Address(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <TestStruct as ::optionable::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(TestEnumOpt::Address(other_0))?;
                }
            }
        }
        Ok(())
    }
}
