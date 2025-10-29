pub enum JSONSchemaPropsOrBoolOpt {
    Schema(
        Option<
            <std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Bool(Option<bool>),
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool {
    type Optioned = JSONSchemaPropsOrBoolOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrBoolOpt {
    type Optioned = JSONSchemaPropsOrBoolOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool {
    fn into_optioned(self) -> JSONSchemaPropsOrBoolOpt {
        match self {
            Self::Schema(self_0) => {
                JSONSchemaPropsOrBoolOpt::Schema(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Bool(self_0) => JSONSchemaPropsOrBoolOpt::Bool(Some(self_0)),
        }
    }
    fn try_from_optioned(
        other: JSONSchemaPropsOrBoolOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrBoolOpt::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrBoolOpt::Bool(other_0) => {
                    Self::Bool(
                        other_0
                            .ok_or(crate::optionable::Error {
                                missing_field: "0",
                            })?,
                    )
                }
            },
        )
    }
    fn merge(
        &mut self,
        other: JSONSchemaPropsOrBoolOpt,
    ) -> Result<(), crate::optionable::Error> {
        match other {
            JSONSchemaPropsOrBoolOpt::Schema(other_0) => {
                if let Self::Schema(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrBoolOpt::Schema(other_0),
                    )?;
                }
            }
            JSONSchemaPropsOrBoolOpt::Bool(other_0) => {
                if let Self::Bool(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrBoolOpt::Bool(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
