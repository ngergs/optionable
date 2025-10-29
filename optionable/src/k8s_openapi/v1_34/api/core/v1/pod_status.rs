pub struct PodStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodCondition>,
    > as crate::Optionable>::Optioned,
    pub container_statuses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerStatus>,
    > as crate::Optionable>::Optioned,
    pub ephemeral_container_statuses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerStatus>,
    > as crate::Optionable>::Optioned,
    pub extended_resource_claim_status: <Option<
        ::k8s_openapi::api::core::v1::PodExtendedResourceClaimStatus,
    > as crate::Optionable>::Optioned,
    pub host_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub host_ips: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::HostIP>,
    > as crate::Optionable>::Optioned,
    pub init_container_statuses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerStatus>,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub nominated_node_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pod_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pod_ips: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodIP>,
    > as crate::Optionable>::Optioned,
    pub qos_class: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resize: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource_claim_statuses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodResourceClaimStatus>,
    > as crate::Optionable>::Optioned,
    pub start_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodStatus {
    type Optioned = PodStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PodStatusOpt {
    type Optioned = PodStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodStatus {
    fn into_optioned(self) -> PodStatusOpt {
        PodStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            container_statuses: crate::OptionableConvert::into_optioned(
                self.container_statuses,
            ),
            ephemeral_container_statuses: crate::OptionableConvert::into_optioned(
                self.ephemeral_container_statuses,
            ),
            extended_resource_claim_status: crate::OptionableConvert::into_optioned(
                self.extended_resource_claim_status,
            ),
            host_ip: crate::OptionableConvert::into_optioned(self.host_ip),
            host_ips: crate::OptionableConvert::into_optioned(self.host_ips),
            init_container_statuses: crate::OptionableConvert::into_optioned(
                self.init_container_statuses,
            ),
            message: crate::OptionableConvert::into_optioned(self.message),
            nominated_node_name: crate::OptionableConvert::into_optioned(
                self.nominated_node_name,
            ),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            phase: crate::OptionableConvert::into_optioned(self.phase),
            pod_ip: crate::OptionableConvert::into_optioned(self.pod_ip),
            pod_ips: crate::OptionableConvert::into_optioned(self.pod_ips),
            qos_class: crate::OptionableConvert::into_optioned(self.qos_class),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            resize: crate::OptionableConvert::into_optioned(self.resize),
            resource_claim_statuses: crate::OptionableConvert::into_optioned(
                self.resource_claim_statuses,
            ),
            start_time: crate::OptionableConvert::into_optioned(self.start_time),
        }
    }
    fn try_from_optioned(value: PodStatusOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            container_statuses: crate::OptionableConvert::try_from_optioned(
                value.container_statuses,
            )?,
            ephemeral_container_statuses: crate::OptionableConvert::try_from_optioned(
                value.ephemeral_container_statuses,
            )?,
            extended_resource_claim_status: crate::OptionableConvert::try_from_optioned(
                value.extended_resource_claim_status,
            )?,
            host_ip: crate::OptionableConvert::try_from_optioned(value.host_ip)?,
            host_ips: crate::OptionableConvert::try_from_optioned(value.host_ips)?,
            init_container_statuses: crate::OptionableConvert::try_from_optioned(
                value.init_container_statuses,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            nominated_node_name: crate::OptionableConvert::try_from_optioned(
                value.nominated_node_name,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
            pod_ip: crate::OptionableConvert::try_from_optioned(value.pod_ip)?,
            pod_ips: crate::OptionableConvert::try_from_optioned(value.pod_ips)?,
            qos_class: crate::OptionableConvert::try_from_optioned(value.qos_class)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            resize: crate::OptionableConvert::try_from_optioned(value.resize)?,
            resource_claim_statuses: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_statuses,
            )?,
            start_time: crate::OptionableConvert::try_from_optioned(value.start_time)?,
        })
    }
    fn merge(&mut self, other: PodStatusOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.container_statuses,
            other.container_statuses,
        )?;
        crate::OptionableConvert::merge(
            &mut self.ephemeral_container_statuses,
            other.ephemeral_container_statuses,
        )?;
        crate::OptionableConvert::merge(
            &mut self.extended_resource_claim_status,
            other.extended_resource_claim_status,
        )?;
        crate::OptionableConvert::merge(&mut self.host_ip, other.host_ip)?;
        crate::OptionableConvert::merge(&mut self.host_ips, other.host_ips)?;
        crate::OptionableConvert::merge(
            &mut self.init_container_statuses,
            other.init_container_statuses,
        )?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(
            &mut self.nominated_node_name,
            other.nominated_node_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        crate::OptionableConvert::merge(&mut self.pod_ip, other.pod_ip)?;
        crate::OptionableConvert::merge(&mut self.pod_ips, other.pod_ips)?;
        crate::OptionableConvert::merge(&mut self.qos_class, other.qos_class)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.resize, other.resize)?;
        crate::OptionableConvert::merge(
            &mut self.resource_claim_statuses,
            other.resource_claim_statuses,
        )?;
        crate::OptionableConvert::merge(&mut self.start_time, other.start_time)?;
        Ok(())
    }
}
