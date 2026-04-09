#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerStatus contains details for the current status of this container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStatusAc {
    /// AllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_resources: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// AllocatedResourcesStatus represents the status of various resources allocated for this Pod.
    pub allocated_resources_status: Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::ResourceStatus>,
    >,
    /// ContainerID is the ID of the container in the format '\<type\>://\<container_id\>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example "containerd").
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<std::string::String>,
    /// Image is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<std::string::String>,
    /// ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.
    #[serde(rename = "imageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<std::string::String>,
    /// LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state: Option<
        <::k8s_openapi027::api::core::v1::ContainerState as crate::Optionable>::Optioned,
    >,
    /// Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Ready specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).
    ///
    /// The value is typically used to determine whether a container is ready to accept traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::ResourceRequirements as crate::Optionable>::Optioned,
    >,
    /// RestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    /// Started indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    /// State holds details about the container's current condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<
        <::k8s_openapi027::api::core::v1::ContainerState as crate::Optionable>::Optioned,
    >,
    /// User represents user identity information initially attached to the first process of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<
        <::k8s_openapi027::api::core::v1::ContainerUser as crate::Optionable>::Optioned,
    >,
    /// Status of volume mounts.
    pub volume_mounts: Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::VolumeMountStatus>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerStatus {
    type Optioned = ContainerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStatusAc {
    type Optioned = ContainerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerStatus {
    fn into_optioned(self) -> ContainerStatusAc {
        ContainerStatusAc {
            allocated_resources: crate::OptionableConvert::into_optioned(
                self.allocated_resources,
            ),
            allocated_resources_status: self.allocated_resources_status,
            container_id: self.container_id,
            image: Some(self.image),
            image_id: Some(self.image_id),
            last_state: crate::OptionableConvert::into_optioned(self.last_state),
            name: Some(self.name),
            ready: Some(self.ready),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_count: Some(self.restart_count),
            started: self.started,
            state: crate::OptionableConvert::into_optioned(self.state),
            user: crate::OptionableConvert::into_optioned(self.user),
            volume_mounts: self.volume_mounts,
        }
    }
    fn try_from_optioned(value: ContainerStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocated_resources: crate::OptionableConvert::try_from_optioned(
                value.allocated_resources,
            )?,
            allocated_resources_status: value.allocated_resources_status,
            container_id: value.container_id,
            image: value
                .image
                .ok_or(crate::Error {
                    missing_field: "image",
                })?,
            image_id: value
                .image_id
                .ok_or(crate::Error {
                    missing_field: "image_id",
                })?,
            last_state: crate::OptionableConvert::try_from_optioned(value.last_state)?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            ready: value
                .ready
                .ok_or(crate::Error {
                    missing_field: "ready",
                })?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_count: value
                .restart_count
                .ok_or(crate::Error {
                    missing_field: "restart_count",
                })?,
            started: value.started,
            state: crate::OptionableConvert::try_from_optioned(value.state)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
            volume_mounts: value.volume_mounts,
        })
    }
    fn merge(&mut self, other: ContainerStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.allocated_resources,
            other.allocated_resources,
        )?;
        self.allocated_resources_status = other.allocated_resources_status;
        self.container_id = other.container_id;
        if let Some(other_value) = other.image {
            self.image = other_value;
        }
        if let Some(other_value) = other.image_id {
            self.image_id = other_value;
        }
        crate::OptionableConvert::merge(&mut self.last_state, other.last_state)?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.ready {
            self.ready = other_value;
        }
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.restart_count {
            self.restart_count = other_value;
        }
        self.started = other.started;
        crate::OptionableConvert::merge(&mut self.state, other.state)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        self.volume_mounts = other.volume_mounts;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerStatus>
for ContainerStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ContainerStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
