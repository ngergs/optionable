#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<
        <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_unknown_fields: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    type Optioned = CustomResourceDefinitionSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionSpecAc {
    type Optioned = CustomResourceDefinitionSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    fn into_optioned(self) -> CustomResourceDefinitionSpecAc {
        CustomResourceDefinitionSpecAc {
            conversion: crate::OptionableConvert::into_optioned(self.conversion),
            group: Some(crate::OptionableConvert::into_optioned(self.group)),
            names: Some(crate::OptionableConvert::into_optioned(self.names)),
            preserve_unknown_fields: crate::OptionableConvert::into_optioned(
                self.preserve_unknown_fields,
            ),
            scope: Some(crate::OptionableConvert::into_optioned(self.scope)),
            versions: Some(crate::OptionableConvert::into_optioned(self.versions)),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conversion: crate::OptionableConvert::try_from_optioned(value.conversion)?,
            group: crate::OptionableConvert::try_from_optioned(
                value
                    .group
                    .ok_or(crate::Error {
                        missing_field: "group",
                    })?,
            )?,
            names: crate::OptionableConvert::try_from_optioned(
                value
                    .names
                    .ok_or(crate::Error {
                        missing_field: "names",
                    })?,
            )?,
            preserve_unknown_fields: crate::OptionableConvert::try_from_optioned(
                value.preserve_unknown_fields,
            )?,
            scope: crate::OptionableConvert::try_from_optioned(
                value
                    .scope
                    .ok_or(crate::Error {
                        missing_field: "scope",
                    })?,
            )?,
            versions: crate::OptionableConvert::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conversion, other.conversion)?;
        if let Some(other_value) = other.group {
            crate::OptionableConvert::merge(&mut self.group, other_value)?;
        }
        if let Some(other_value) = other.names {
            crate::OptionableConvert::merge(&mut self.names, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.preserve_unknown_fields,
            other.preserve_unknown_fields,
        )?;
        if let Some(other_value) = other.scope {
            crate::OptionableConvert::merge(&mut self.scope, other_value)?;
        }
        if let Some(other_value) = other.versions {
            crate::OptionableConvert::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
> for CustomResourceDefinitionSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
