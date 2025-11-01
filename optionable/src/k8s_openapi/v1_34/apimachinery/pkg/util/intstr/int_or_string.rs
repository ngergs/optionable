pub enum IntOrStringAc {
    Int(Option<i32>),
    String(Option<<std::string::String as crate::Optionable>::Optioned>),
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString {
    type Optioned = IntOrStringAc;
}
#[automatically_derived]
impl crate::Optionable for IntOrStringAc {
    type Optioned = IntOrStringAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString {
    fn into_optioned(self) -> IntOrStringAc {
        match self {
            Self::Int(self_0) => IntOrStringAc::Int(Some(self_0)),
            Self::String(self_0) => {
                IntOrStringAc::String(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(
        other: IntOrStringAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                IntOrStringAc::Int(other_0) => {
                    Self::Int(
                        other_0
                            .ok_or(crate::optionable::Error {
                                missing_field: "0",
                            })?,
                    )
                }
                IntOrStringAc::String(other_0) => {
                    Self::String(
                        crate::OptionableConvert::try_from_optioned(
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
    fn merge(&mut self, other: IntOrStringAc) -> Result<(), crate::optionable::Error> {
        match other {
            IntOrStringAc::Int(other_0) => {
                if let Self::Int(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(IntOrStringAc::Int(other_0))?;
                }
            }
            IntOrStringAc::String(other_0) => {
                if let Self::String(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(IntOrStringAc::String(other_0))?;
                }
            }
        }
        Ok(())
    }
}
