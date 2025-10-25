pub struct NodeSystemInfoOpt {
    pub architecture: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub boot_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub container_runtime_version: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
    pub kernel_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub kube_proxy_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub kubelet_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub machine_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub operating_system: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub os_image: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub swap: <Option<
        ::k8s_openapi::api::core::v1::NodeSwapStatus,
    > as crate::Optionable>::Optioned,
    pub system_uuid: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSystemInfo {
    type Optioned = NodeSystemInfoOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeSystemInfoOpt {
    type Optioned = NodeSystemInfoOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSystemInfo {
    fn into_optioned(self) -> NodeSystemInfoOpt {
        NodeSystemInfoOpt {
            architecture: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.architecture,
                ),
            ),
            boot_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.boot_id,
                ),
            ),
            container_runtime_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.container_runtime_version,
                ),
            ),
            kernel_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kernel_version,
                ),
            ),
            kube_proxy_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kube_proxy_version,
                ),
            ),
            kubelet_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kubelet_version,
                ),
            ),
            machine_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.machine_id,
                ),
            ),
            operating_system: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.operating_system,
                ),
            ),
            os_image: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.os_image,
                ),
            ),
            swap: <Option<
                ::k8s_openapi::api::core::v1::NodeSwapStatus,
            > as crate::OptionableConvert>::into_optioned(self.swap),
            system_uuid: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.system_uuid,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: NodeSystemInfoOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            architecture: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .architecture
                    .ok_or(crate::optionable::Error {
                        missing_field: "architecture",
                    })?,
            )?,
            boot_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .boot_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "boot_id",
                    })?,
            )?,
            container_runtime_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .container_runtime_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "container_runtime_version",
                    })?,
            )?,
            kernel_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kernel_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "kernel_version",
                    })?,
            )?,
            kube_proxy_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kube_proxy_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "kube_proxy_version",
                    })?,
            )?,
            kubelet_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kubelet_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "kubelet_version",
                    })?,
            )?,
            machine_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .machine_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "machine_id",
                    })?,
            )?,
            operating_system: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .operating_system
                    .ok_or(crate::optionable::Error {
                        missing_field: "operating_system",
                    })?,
            )?,
            os_image: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .os_image
                    .ok_or(crate::optionable::Error {
                        missing_field: "os_image",
                    })?,
            )?,
            swap: <Option<
                ::k8s_openapi::api::core::v1::NodeSwapStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.swap)?,
            system_uuid: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .system_uuid
                    .ok_or(crate::optionable::Error {
                        missing_field: "system_uuid",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeSystemInfoOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.architecture {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.architecture,
                other_value,
            )?;
        }
        if let Some(other_value) = other.boot_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.boot_id,
                other_value,
            )?;
        }
        if let Some(other_value) = other.container_runtime_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.container_runtime_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.kernel_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kernel_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.kube_proxy_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kube_proxy_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.kubelet_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kubelet_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.machine_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.machine_id,
                other_value,
            )?;
        }
        if let Some(other_value) = other.operating_system {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.operating_system,
                other_value,
            )?;
        }
        if let Some(other_value) = other.os_image {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.os_image,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::NodeSwapStatus,
        > as crate::OptionableConvert>::merge(&mut self.swap, other.swap)?;
        if let Some(other_value) = other.system_uuid {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.system_uuid,
                other_value,
            )?;
        }
        Ok(())
    }
}
