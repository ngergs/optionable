#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_resources: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_resources_status: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ResourceStatus>,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "imageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state: <Option<
        ::k8s_openapi::api::core::v1::ContainerState,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: <Option<
        ::k8s_openapi::api::core::v1::ResourceRequirements,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: <Option<
        ::k8s_openapi::api::core::v1::ContainerState,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<
        ::k8s_openapi::api::core::v1::ContainerUser,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMountStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerStatus {
    type Optioned = ContainerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStatusAc {
    type Optioned = ContainerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerStatus {
    fn into_optioned(self) -> ContainerStatusAc {
        ContainerStatusAc {
            allocated_resources: crate::OptionableConvert::into_optioned(
                self.allocated_resources,
            ),
            allocated_resources_status: crate::OptionableConvert::into_optioned(
                self.allocated_resources_status,
            ),
            container_id: crate::OptionableConvert::into_optioned(self.container_id),
            image: Some(crate::OptionableConvert::into_optioned(self.image)),
            image_id: Some(crate::OptionableConvert::into_optioned(self.image_id)),
            last_state: crate::OptionableConvert::into_optioned(self.last_state),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            ready: Some(self.ready),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_count: Some(self.restart_count),
            started: crate::OptionableConvert::into_optioned(self.started),
            state: crate::OptionableConvert::into_optioned(self.state),
            user: crate::OptionableConvert::into_optioned(self.user),
            volume_mounts: crate::OptionableConvert::into_optioned(self.volume_mounts),
        }
    }
    fn try_from_optioned(value: ContainerStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocated_resources: crate::OptionableConvert::try_from_optioned(
                value.allocated_resources,
            )?,
            allocated_resources_status: crate::OptionableConvert::try_from_optioned(
                value.allocated_resources_status,
            )?,
            container_id: crate::OptionableConvert::try_from_optioned(
                value.container_id,
            )?,
            image: crate::OptionableConvert::try_from_optioned(
                value
                    .image
                    .ok_or(crate::Error {
                        missing_field: "image",
                    })?,
            )?,
            image_id: crate::OptionableConvert::try_from_optioned(
                value
                    .image_id
                    .ok_or(crate::Error {
                        missing_field: "image_id",
                    })?,
            )?,
            last_state: crate::OptionableConvert::try_from_optioned(value.last_state)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
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
            started: crate::OptionableConvert::try_from_optioned(value.started)?,
            state: crate::OptionableConvert::try_from_optioned(value.state)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
            volume_mounts: crate::OptionableConvert::try_from_optioned(
                value.volume_mounts,
            )?,
        })
    }
    fn merge(&mut self, other: ContainerStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.allocated_resources,
            other.allocated_resources,
        )?;
        crate::OptionableConvert::merge(
            &mut self.allocated_resources_status,
            other.allocated_resources_status,
        )?;
        crate::OptionableConvert::merge(&mut self.container_id, other.container_id)?;
        if let Some(other_value) = other.image {
            crate::OptionableConvert::merge(&mut self.image, other_value)?;
        }
        if let Some(other_value) = other.image_id {
            crate::OptionableConvert::merge(&mut self.image_id, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.last_state, other.last_state)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.ready {
            self.ready = other_value;
        }
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.restart_count {
            self.restart_count = other_value;
        }
        crate::OptionableConvert::merge(&mut self.started, other.started)?;
        crate::OptionableConvert::merge(&mut self.state, other.state)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        crate::OptionableConvert::merge(&mut self.volume_mounts, other.volume_mounts)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ContainerStatus>
for ContainerStatusAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::ContainerStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ContainerStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ContainerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
