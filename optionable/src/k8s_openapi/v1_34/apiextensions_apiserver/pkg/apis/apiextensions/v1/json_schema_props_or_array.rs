pub enum JSONSchemaPropsOrArrayOpt {
    Schema(
        Option<
            <std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Schemas(
        Option<
            <std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray {
    type Optioned = JSONSchemaPropsOrArrayOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrArrayOpt {
    type Optioned = JSONSchemaPropsOrArrayOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray {
    fn into_optioned(self) -> JSONSchemaPropsOrArrayOpt {
        match self {
            Self::Schema(self_0) => {
                JSONSchemaPropsOrArrayOpt::Schema(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Schemas(self_0) => {
                JSONSchemaPropsOrArrayOpt::Schemas(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(
        other: JSONSchemaPropsOrArrayOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrArrayOpt::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrArrayOpt::Schemas(other_0) => {
                    Self::Schemas(
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
        other: JSONSchemaPropsOrArrayOpt,
    ) -> Result<(), crate::optionable::Error> {
        match other {
            JSONSchemaPropsOrArrayOpt::Schema(other_0) => {
                if let Self::Schema(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrArrayOpt::Schema(other_0),
                    )?;
                }
            }
            JSONSchemaPropsOrArrayOpt::Schemas(other_0) => {
                if let Self::Schemas(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrArrayOpt::Schemas(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
