#[derive(Clone, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JSONSchemaPropsOrBoolAc {
    Schema(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Bool(#[serde(skip_serializing_if = "Option::is_none")] Option<bool>),
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool {
    type Optioned = JSONSchemaPropsOrBoolAc;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrBoolAc {
    type Optioned = JSONSchemaPropsOrBoolAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool {
    fn into_optioned(self) -> JSONSchemaPropsOrBoolAc {
        match self {
            Self::Schema(self_0) => {
                JSONSchemaPropsOrBoolAc::Schema(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Bool(self_0) => JSONSchemaPropsOrBoolAc::Bool(Some(self_0)),
        }
    }
    fn try_from_optioned(
        other: JSONSchemaPropsOrBoolAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrBoolAc::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrBoolAc::Bool(other_0) => {
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
        other: JSONSchemaPropsOrBoolAc,
    ) -> Result<(), crate::optionable::Error> {
        match other {
            JSONSchemaPropsOrBoolAc::Schema(other_0) => {
                if let Self::Schema(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrBoolAc::Schema(other_0),
                    )?;
                }
            }
            JSONSchemaPropsOrBoolAc::Bool(other_0) => {
                if let Self::Bool(self_0) = self {
                    if let Some(other_value) = other_0 {
                        *self_0 = other_value;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrBoolAc::Bool(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
