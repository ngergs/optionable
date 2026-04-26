#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ObjectFieldSelector selects an APIVersioned field of an object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectFieldSelectorAc {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// Path of the field to select in the specified API version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ObjectFieldSelector {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectFieldSelectorAc {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ObjectFieldSelector {
    fn into_optioned(self) -> ObjectFieldSelectorAc {
        ObjectFieldSelectorAc {
            api_version: self.api_version,
            field_path: Some(self.field_path),
        }
    }
    fn try_from_optioned(value: ObjectFieldSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            field_path: value
                .field_path
                .ok_or(crate::Error {
                    missing_field: "field_path",
                })?,
        })
    }
    fn merge(&mut self, other: ObjectFieldSelectorAc) -> Result<(), crate::Error> {
        if self.api_version.is_none() {
            self.api_version = crate::OptionableConvert::try_from_optioned(
                other.api_version,
            )?;
        } else if let Some(self_value) = self.api_version.as_mut()
            && let Some(other_value) = other.api_version
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.field_path {
            self.field_path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ObjectFieldSelector>
for ObjectFieldSelectorAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ObjectFieldSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ObjectFieldSelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ObjectFieldSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ObjectFieldSelectorAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.field_path, other.field_path);
    }
}
