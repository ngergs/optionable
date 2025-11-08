#[derive(Clone, std::fmt::Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum JSONSchemaPropsOrArrayAc {
    Schema(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Schemas(
        #[serde(skip_serializing_if = "Option::is_none")]
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
    type Optioned = JSONSchemaPropsOrArrayAc;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrArrayAc {
    type Optioned = JSONSchemaPropsOrArrayAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray {
    fn into_optioned(self) -> JSONSchemaPropsOrArrayAc {
        match self {
            Self::Schema(self_0) => {
                JSONSchemaPropsOrArrayAc::Schema(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Schemas(self_0) => {
                JSONSchemaPropsOrArrayAc::Schemas(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(other: JSONSchemaPropsOrArrayAc) -> Result<Self, crate::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrArrayAc::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrArrayAc::Schemas(other_0) => {
                    Self::Schemas(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: JSONSchemaPropsOrArrayAc) -> Result<(), crate::Error> {
        match other {
            JSONSchemaPropsOrArrayAc::Schema(other_0) => {
                if let Self::Schema(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrArrayAc::Schema(other_0),
                    )?;
                }
            }
            JSONSchemaPropsOrArrayAc::Schemas(other_0) => {
                if let Self::Schemas(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        JSONSchemaPropsOrArrayAc::Schemas(other_0),
                    )?;
                }
            }
        }
        Ok(())
    }
}
