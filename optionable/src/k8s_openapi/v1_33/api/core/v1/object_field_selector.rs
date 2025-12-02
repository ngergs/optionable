#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ObjectFieldSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ObjectFieldSelector {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectFieldSelectorAc {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ObjectFieldSelector {
    fn into_optioned(self) -> ObjectFieldSelectorAc {
        ObjectFieldSelectorAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            field_path: Some(crate::OptionableConvert::into_optioned(self.field_path)),
        }
    }
    fn try_from_optioned(value: ObjectFieldSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            field_path: crate::OptionableConvert::try_from_optioned(
                value
                    .field_path
                    .ok_or(crate::Error {
                        missing_field: "field_path",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ObjectFieldSelectorAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.field_path {
            crate::OptionableConvert::merge(&mut self.field_path, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ObjectFieldSelector>
for ObjectFieldSelectorAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ObjectFieldSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ObjectFieldSelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ObjectFieldSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
