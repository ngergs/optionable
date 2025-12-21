#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize, std::fmt::Debug)]
#[serde(rename_all_fields = "camelCase", deny_unknown_fields)]
#[serde(untagged)]
pub enum JSONSchemaPropsOrStringArrayAc {
    Schema(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <std::boxed::Box<
                ::k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            > as crate::Optionable>::Optioned,
        >,
    ),
    Strings(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<std::vec::Vec<std::string::String> as crate::Optionable>::Optioned>,
    ),
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray {
    type Optioned = JSONSchemaPropsOrStringArrayAc;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOrStringArrayAc {
    type Optioned = JSONSchemaPropsOrStringArrayAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray {
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
    ) -> Result<Self, crate::Error> {
        Ok(
            match other {
                JSONSchemaPropsOrStringArrayAc::Schema(other_0) => {
                    Self::Schema(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                JSONSchemaPropsOrStringArrayAc::Strings(other_0) => {
                    Self::Strings(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(
        &mut self,
        other: JSONSchemaPropsOrStringArrayAc,
    ) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
> for JSONSchemaPropsOrStringArrayAc {
    fn from_optionable(
        value: k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
