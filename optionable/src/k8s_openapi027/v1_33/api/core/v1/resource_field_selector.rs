#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceFieldSelectorAc {
    /// Container name: required for volumes, optional for env vars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<std::string::String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisor: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Required: resource to select
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceFieldSelector {
    type Optioned = ResourceFieldSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceFieldSelectorAc {
    type Optioned = ResourceFieldSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceFieldSelector {
    fn into_optioned(self) -> ResourceFieldSelectorAc {
        ResourceFieldSelectorAc {
            container_name: self.container_name,
            divisor: crate::OptionableConvert::into_optioned(self.divisor),
            resource: Some(self.resource),
        }
    }
    fn try_from_optioned(value: ResourceFieldSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            container_name: value.container_name,
            divisor: crate::OptionableConvert::try_from_optioned(value.divisor)?,
            resource: value
                .resource
                .ok_or(crate::Error {
                    missing_field: "resource",
                })?,
        })
    }
    fn merge(&mut self, other: ResourceFieldSelectorAc) -> Result<(), crate::Error> {
        self.container_name = other.container_name;
        crate::OptionableConvert::merge(&mut self.divisor, other.divisor)?;
        if let Some(other_value) = other.resource {
            self.resource = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceFieldSelector>
for ResourceFieldSelectorAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ResourceFieldSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceFieldSelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceFieldSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
