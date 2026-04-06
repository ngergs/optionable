#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSystemInfoAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "bootID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_runtime_version: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kube_proxy_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "machineID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_image: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "systemUUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_uuid: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSystemInfo {
    type Optioned = NodeSystemInfoAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSystemInfoAc {
    type Optioned = NodeSystemInfoAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeSystemInfo {
    fn into_optioned(self) -> NodeSystemInfoAc {
        NodeSystemInfoAc {
            architecture: Some(
                crate::OptionableConvert::into_optioned(self.architecture),
            ),
            boot_id: Some(crate::OptionableConvert::into_optioned(self.boot_id)),
            container_runtime_version: Some(
                crate::OptionableConvert::into_optioned(self.container_runtime_version),
            ),
            kernel_version: Some(
                crate::OptionableConvert::into_optioned(self.kernel_version),
            ),
            kube_proxy_version: Some(
                crate::OptionableConvert::into_optioned(self.kube_proxy_version),
            ),
            kubelet_version: Some(
                crate::OptionableConvert::into_optioned(self.kubelet_version),
            ),
            machine_id: Some(crate::OptionableConvert::into_optioned(self.machine_id)),
            operating_system: Some(
                crate::OptionableConvert::into_optioned(self.operating_system),
            ),
            os_image: Some(crate::OptionableConvert::into_optioned(self.os_image)),
            system_uuid: Some(crate::OptionableConvert::into_optioned(self.system_uuid)),
        }
    }
    fn try_from_optioned(value: NodeSystemInfoAc) -> Result<Self, crate::Error> {
        Ok(Self {
            architecture: crate::OptionableConvert::try_from_optioned(
                value
                    .architecture
                    .ok_or(crate::Error {
                        missing_field: "architecture",
                    })?,
            )?,
            boot_id: crate::OptionableConvert::try_from_optioned(
                value
                    .boot_id
                    .ok_or(crate::Error {
                        missing_field: "boot_id",
                    })?,
            )?,
            container_runtime_version: crate::OptionableConvert::try_from_optioned(
                value
                    .container_runtime_version
                    .ok_or(crate::Error {
                        missing_field: "container_runtime_version",
                    })?,
            )?,
            kernel_version: crate::OptionableConvert::try_from_optioned(
                value
                    .kernel_version
                    .ok_or(crate::Error {
                        missing_field: "kernel_version",
                    })?,
            )?,
            kube_proxy_version: crate::OptionableConvert::try_from_optioned(
                value
                    .kube_proxy_version
                    .ok_or(crate::Error {
                        missing_field: "kube_proxy_version",
                    })?,
            )?,
            kubelet_version: crate::OptionableConvert::try_from_optioned(
                value
                    .kubelet_version
                    .ok_or(crate::Error {
                        missing_field: "kubelet_version",
                    })?,
            )?,
            machine_id: crate::OptionableConvert::try_from_optioned(
                value
                    .machine_id
                    .ok_or(crate::Error {
                        missing_field: "machine_id",
                    })?,
            )?,
            operating_system: crate::OptionableConvert::try_from_optioned(
                value
                    .operating_system
                    .ok_or(crate::Error {
                        missing_field: "operating_system",
                    })?,
            )?,
            os_image: crate::OptionableConvert::try_from_optioned(
                value
                    .os_image
                    .ok_or(crate::Error {
                        missing_field: "os_image",
                    })?,
            )?,
            system_uuid: crate::OptionableConvert::try_from_optioned(
                value
                    .system_uuid
                    .ok_or(crate::Error {
                        missing_field: "system_uuid",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NodeSystemInfoAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.architecture {
            crate::OptionableConvert::merge(&mut self.architecture, other_value)?;
        }
        if let Some(other_value) = other.boot_id {
            crate::OptionableConvert::merge(&mut self.boot_id, other_value)?;
        }
        if let Some(other_value) = other.container_runtime_version {
            crate::OptionableConvert::merge(
                &mut self.container_runtime_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.kernel_version {
            crate::OptionableConvert::merge(&mut self.kernel_version, other_value)?;
        }
        if let Some(other_value) = other.kube_proxy_version {
            crate::OptionableConvert::merge(&mut self.kube_proxy_version, other_value)?;
        }
        if let Some(other_value) = other.kubelet_version {
            crate::OptionableConvert::merge(&mut self.kubelet_version, other_value)?;
        }
        if let Some(other_value) = other.machine_id {
            crate::OptionableConvert::merge(&mut self.machine_id, other_value)?;
        }
        if let Some(other_value) = other.operating_system {
            crate::OptionableConvert::merge(&mut self.operating_system, other_value)?;
        }
        if let Some(other_value) = other.os_image {
            crate::OptionableConvert::merge(&mut self.os_image, other_value)?;
        }
        if let Some(other_value) = other.system_uuid {
            crate::OptionableConvert::merge(&mut self.system_uuid, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSystemInfo>
for NodeSystemInfoAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeSystemInfo) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSystemInfo, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSystemInfo,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
