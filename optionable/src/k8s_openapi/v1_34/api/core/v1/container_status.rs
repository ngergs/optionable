pub struct ContainerStatusOpt {
    pub allocated_resources: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub allocated_resources_status: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ResourceStatus>,
    > as crate::Optionable>::Optioned,
    pub container_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub image: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub image_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub last_state: <Option<
        ::k8s_openapi::api::core::v1::ContainerState,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub ready: Option<bool>,
    pub resources: <Option<
        ::k8s_openapi::api::core::v1::ResourceRequirements,
    > as crate::Optionable>::Optioned,
    pub restart_count: Option<i32>,
    pub started: <Option<bool> as crate::Optionable>::Optioned,
    pub state: <Option<
        ::k8s_openapi::api::core::v1::ContainerState,
    > as crate::Optionable>::Optioned,
    pub stop_signal: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<
        ::k8s_openapi::api::core::v1::ContainerUser,
    > as crate::Optionable>::Optioned,
    pub volume_mounts: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMountStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_status::ContainerStatus {
    type Optioned = ContainerStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerStatusOpt {
    type Optioned = ContainerStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_status::ContainerStatus {
    fn into_optioned(self) -> ContainerStatusOpt {
        ContainerStatusOpt {
            allocated_resources: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.allocated_resources),
            allocated_resources_status: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ResourceStatus>,
            > as crate::OptionableConvert>::into_optioned(
                self.allocated_resources_status,
            ),
            container_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.container_id),
            image: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.image,
                ),
            ),
            image_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.image_id,
                ),
            ),
            last_state: <Option<
                ::k8s_openapi::api::core::v1::ContainerState,
            > as crate::OptionableConvert>::into_optioned(self.last_state),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            ready: Some(self.ready),
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::into_optioned(self.resources),
            restart_count: Some(self.restart_count),
            started: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.started),
            state: <Option<
                ::k8s_openapi::api::core::v1::ContainerState,
            > as crate::OptionableConvert>::into_optioned(self.state),
            stop_signal: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.stop_signal),
            user: <Option<
                ::k8s_openapi::api::core::v1::ContainerUser,
            > as crate::OptionableConvert>::into_optioned(self.user),
            volume_mounts: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMountStatus>,
            > as crate::OptionableConvert>::into_optioned(self.volume_mounts),
        }
    }
    fn try_from_optioned(
        value: ContainerStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocated_resources: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allocated_resources,
            )?,
            allocated_resources_status: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ResourceStatus>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allocated_resources_status,
            )?,
            container_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.container_id)?,
            image: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .image
                    .ok_or(crate::optionable::Error {
                        missing_field: "image",
                    })?,
            )?,
            image_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .image_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "image_id",
                    })?,
            )?,
            last_state: <Option<
                ::k8s_openapi::api::core::v1::ContainerState,
            > as crate::OptionableConvert>::try_from_optioned(value.last_state)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            ready: value
                .ready
                .ok_or(crate::optionable::Error {
                    missing_field: "ready",
                })?,
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::try_from_optioned(value.resources)?,
            restart_count: value
                .restart_count
                .ok_or(crate::optionable::Error {
                    missing_field: "restart_count",
                })?,
            started: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.started)?,
            state: <Option<
                ::k8s_openapi::api::core::v1::ContainerState,
            > as crate::OptionableConvert>::try_from_optioned(value.state)?,
            stop_signal: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.stop_signal)?,
            user: <Option<
                ::k8s_openapi::api::core::v1::ContainerUser,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
            volume_mounts: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMountStatus>,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_mounts)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.allocated_resources,
            other.allocated_resources,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ResourceStatus>,
        > as crate::OptionableConvert>::merge(
            &mut self.allocated_resources_status,
            other.allocated_resources_status,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.container_id,
            other.container_id,
        )?;
        if let Some(other_value) = other.image {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.image,
                other_value,
            )?;
        }
        if let Some(other_value) = other.image_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.image_id,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ContainerState,
        > as crate::OptionableConvert>::merge(&mut self.last_state, other.last_state)?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.ready {
            self.ready = other_value;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ResourceRequirements,
        > as crate::OptionableConvert>::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.restart_count {
            self.restart_count = other_value;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.started, other.started)?;
        <Option<
            ::k8s_openapi::api::core::v1::ContainerState,
        > as crate::OptionableConvert>::merge(&mut self.state, other.state)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.stop_signal, other.stop_signal)?;
        <Option<
            ::k8s_openapi::api::core::v1::ContainerUser,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMountStatus>,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_mounts,
            other.volume_mounts,
        )?;
        Ok(())
    }
}
