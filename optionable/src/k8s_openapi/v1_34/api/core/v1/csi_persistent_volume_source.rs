pub struct CSIPersistentVolumeSourceOpt {
    pub controller_expand_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub controller_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_expand_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub node_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub node_stage_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub volume_attributes: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub volume_handle: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource {
    type Optioned = CSIPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for CSIPersistentVolumeSourceOpt {
    type Optioned = CSIPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource {
    fn into_optioned(self) -> CSIPersistentVolumeSourceOpt {
        CSIPersistentVolumeSourceOpt {
            controller_expand_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(
                self.controller_expand_secret_ref,
            ),
            controller_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(
                self.controller_publish_secret_ref,
            ),
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            node_expand_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(self.node_expand_secret_ref),
            node_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(self.node_publish_secret_ref),
            node_stage_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(self.node_stage_secret_ref),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            volume_attributes: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.volume_attributes),
            volume_handle: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.volume_handle,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CSIPersistentVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            controller_expand_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.controller_expand_secret_ref,
            )?,
            controller_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.controller_publish_secret_ref,
            )?,
            driver: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            node_expand_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.node_expand_secret_ref,
            )?,
            node_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            node_stage_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.node_stage_secret_ref,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            volume_attributes: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_attributes)?,
            volume_handle: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .volume_handle
                    .ok_or(crate::optionable::Error {
                        missing_field: "volume_handle",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CSIPersistentVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(
            &mut self.controller_expand_secret_ref,
            other.controller_expand_secret_ref,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(
            &mut self.controller_publish_secret_ref,
            other.controller_publish_secret_ref,
        )?;
        if let Some(other_value) = other.driver {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.driver,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(
            &mut self.node_expand_secret_ref,
            other.node_expand_secret_ref,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(
            &mut self.node_publish_secret_ref,
            other.node_publish_secret_ref,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(
            &mut self.node_stage_secret_ref,
            other.node_stage_secret_ref,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_attributes,
            other.volume_attributes,
        )?;
        if let Some(other_value) = other.volume_handle {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.volume_handle,
                other_value,
            )?;
        }
        Ok(())
    }
}
