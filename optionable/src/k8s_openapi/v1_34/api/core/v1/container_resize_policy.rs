#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResizePolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerResizePolicy {
    type Optioned = ContainerResizePolicyAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerResizePolicyAc {
    type Optioned = ContainerResizePolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerResizePolicy {
    fn into_optioned(self) -> ContainerResizePolicyAc {
        ContainerResizePolicyAc {
            resource_name: Some(
                crate::OptionableConvert::into_optioned(self.resource_name),
            ),
            restart_policy: Some(
                crate::OptionableConvert::into_optioned(self.restart_policy),
            ),
        }
    }
    fn try_from_optioned(value: ContainerResizePolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            resource_name: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_name
                    .ok_or(crate::Error {
                        missing_field: "resource_name",
                    })?,
            )?,
            restart_policy: crate::OptionableConvert::try_from_optioned(
                value
                    .restart_policy
                    .ok_or(crate::Error {
                        missing_field: "restart_policy",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ContainerResizePolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.resource_name {
            crate::OptionableConvert::merge(&mut self.resource_name, other_value)?;
        }
        if let Some(other_value) = other.restart_policy {
            crate::OptionableConvert::merge(&mut self.restart_policy, other_value)?;
        }
        Ok(())
    }
}
