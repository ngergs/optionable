pub enum JSONSchemaPropsOrStringArrayAc {
    Schema(
        Option<
            <std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Strings(Option<<std::vec::Vec<std::string::String> as crate::Optionable>::Optioned>),
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray {
    type Optioned = JSONSchemaPropsOrStringArrayAc;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrStringArrayAc {
    type Optioned = JSONSchemaPropsOrStringArrayAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray {
    fn into_optioned(self) -> JSONSchemaPropsOrStringArrayAc {
        match self {
            Self::Schema(self_0) => {
                JSONSchemaPropsOrStringArrayAc::Schema(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Strings(self_0) => {
                JSONSchemaPropsOrStringArrayAc::Strings(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(
        other: JSONSchemaPropsOrStringArrayAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrStringArrayAc::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrStringArrayAc::Strings(other_0) => {
                    Self::Strings(
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
    fn merge(
        &mut self,
        other: JSONSchemaPropsOrStringArrayAc,
    ) -> Result<(), crate::optionable::Error> {
        match other {
            JSONSchemaPropsOrStringArrayAc::Schema(other_0) => {
                if let Self::Schema(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrStringArrayAc::Schema(other_0),
                    )?;
                }
            }
            JSONSchemaPropsOrStringArrayAc::Strings(other_0) => {
                if let Self::Strings(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrStringArrayAc::Strings(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
