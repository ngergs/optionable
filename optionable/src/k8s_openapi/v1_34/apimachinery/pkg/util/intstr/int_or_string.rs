pub enum IntOrStringOpt {
    Int(Option<i32>),
    String(Option<<std::string::String as crate::Optionable>::Optioned>),
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString {
    type Optioned = IntOrStringOpt;
}
#[automatically_derived]
impl crate::Optionable for IntOrStringOpt {
    type Optioned = IntOrStringOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString {
    fn into_optioned(self) -> IntOrStringOpt {
        match self {
            Self::Int(self_0) => IntOrStringOpt::Int(Some(self_0)),
            Self::String(self_0) => {
                IntOrStringOpt::String(
                    Some(
                        <std::string::String as crate::OptionableConvert>::into_optioned(
                            self_0,
                        ),
                    ),
                )
            }
        }
    }
    fn try_from_optioned(
        other: IntOrStringOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                IntOrStringOpt::Int(other_0) => {
                    Self::Int(
                        other_0
                            .ok_or(crate::optionable::Error {
                                missing_field: "0",
                            })?,
                    )
                }
                IntOrStringOpt::String(other_0) => {
                    Self::String(
                        <std::string::String as crate::OptionableConvert>::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: IntOrStringOpt) -> Result<(), crate::optionable::Error> {
        match other {
            IntOrStringOpt::Int(other_0) => {
                if let Self::Int(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(IntOrStringOpt::Int(other_0))?;
                }
            }
            IntOrStringOpt::String(other_0) => {
                if let Self::String(self_0) = self {
                    if let Some(other_value) = other_0 {
                        <std::string::String as crate::OptionableConvert>::merge(
                            self_0,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(IntOrStringOpt::String(other_0))?;
                }
            }
        }
        Ok(())
    }
}
