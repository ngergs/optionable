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
    pub architecture: Option<std::string::String>,
    #[serde(rename = "bootID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_runtime_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kube_proxy_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<std::string::String>,
    #[serde(rename = "machineID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_image: Option<std::string::String>,
    #[serde(rename = "systemUUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_uuid: Option<std::string::String>,
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
            architecture: Some(self.architecture),
            boot_id: Some(self.boot_id),
            container_runtime_version: Some(self.container_runtime_version),
            kernel_version: Some(self.kernel_version),
            kube_proxy_version: Some(self.kube_proxy_version),
            kubelet_version: Some(self.kubelet_version),
            machine_id: Some(self.machine_id),
            operating_system: Some(self.operating_system),
            os_image: Some(self.os_image),
            system_uuid: Some(self.system_uuid),
        }
    }
    fn try_from_optioned(value: NodeSystemInfoAc) -> Result<Self, crate::Error> {
        Ok(Self {
            architecture: value
                .architecture
                .ok_or(crate::Error {
                    missing_field: "architecture",
                })?,
            boot_id: value
                .boot_id
                .ok_or(crate::Error {
                    missing_field: "boot_id",
                })?,
            container_runtime_version: value
                .container_runtime_version
                .ok_or(crate::Error {
                    missing_field: "container_runtime_version",
                })?,
            kernel_version: value
                .kernel_version
                .ok_or(crate::Error {
                    missing_field: "kernel_version",
                })?,
            kube_proxy_version: value
                .kube_proxy_version
                .ok_or(crate::Error {
                    missing_field: "kube_proxy_version",
                })?,
            kubelet_version: value
                .kubelet_version
                .ok_or(crate::Error {
                    missing_field: "kubelet_version",
                })?,
            machine_id: value
                .machine_id
                .ok_or(crate::Error {
                    missing_field: "machine_id",
                })?,
            operating_system: value
                .operating_system
                .ok_or(crate::Error {
                    missing_field: "operating_system",
                })?,
            os_image: value
                .os_image
                .ok_or(crate::Error {
                    missing_field: "os_image",
                })?,
            system_uuid: value
                .system_uuid
                .ok_or(crate::Error {
                    missing_field: "system_uuid",
                })?,
        })
    }
    fn merge(&mut self, other: NodeSystemInfoAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.architecture {
            self.architecture = other_value;
        }
        if let Some(other_value) = other.boot_id {
            self.boot_id = other_value;
        }
        if let Some(other_value) = other.container_runtime_version {
            self.container_runtime_version = other_value;
        }
        if let Some(other_value) = other.kernel_version {
            self.kernel_version = other_value;
        }
        if let Some(other_value) = other.kube_proxy_version {
            self.kube_proxy_version = other_value;
        }
        if let Some(other_value) = other.kubelet_version {
            self.kubelet_version = other_value;
        }
        if let Some(other_value) = other.machine_id {
            self.machine_id = other_value;
        }
        if let Some(other_value) = other.operating_system {
            self.operating_system = other_value;
        }
        if let Some(other_value) = other.os_image {
            self.os_image = other_value;
        }
        if let Some(other_value) = other.system_uuid {
            self.system_uuid = other_value;
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
