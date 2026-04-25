#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerResizePolicy represents resource resize policy for the container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerResizePolicyAc {
    /// Name of the resource to which this resource resize policy applies. Supported values: cpu, memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<std::string::String>,
    /// Restart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerResizePolicy {
    type Optioned = ContainerResizePolicyAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerResizePolicyAc {
    type Optioned = ContainerResizePolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerResizePolicy {
    fn into_optioned(self) -> ContainerResizePolicyAc {
        ContainerResizePolicyAc {
            resource_name: Some(self.resource_name),
            restart_policy: Some(self.restart_policy),
        }
    }
    fn try_from_optioned(value: ContainerResizePolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            resource_name: value
                .resource_name
                .ok_or(crate::Error {
                    missing_field: "resource_name",
                })?,
            restart_policy: value
                .restart_policy
                .ok_or(crate::Error {
                    missing_field: "restart_policy",
                })?,
        })
    }
    fn merge(&mut self, other: ContainerResizePolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.resource_name {
            self.resource_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.restart_policy {
            self.restart_policy = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerResizePolicy>
for ContainerResizePolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerResizePolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerResizePolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerResizePolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ContainerResizePolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.resource_name,
            other.resource_name,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.restart_policy,
            other.restart_policy,
        );
    }
}
