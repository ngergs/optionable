pub struct CSIVolumeSourceOpt {
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub volume_attributes: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::csi_volume_source::CSIVolumeSource {
    type Optioned = CSIVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for CSIVolumeSourceOpt {
    type Optioned = CSIVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::csi_volume_source::CSIVolumeSource {
    fn into_optioned(self) -> CSIVolumeSourceOpt {
        CSIVolumeSourceOpt {
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            node_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.node_publish_secret_ref),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            volume_attributes: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.volume_attributes),
        }
    }
    fn try_from_optioned(
        value: CSIVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
            node_publish_secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            volume_attributes: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_attributes)?,
        })
    }
    fn merge(
        &mut self,
        other: CSIVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
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
            ::k8s_openapi::api::core::v1::LocalObjectReference,
        > as crate::OptionableConvert>::merge(
            &mut self.node_publish_secret_ref,
            other.node_publish_secret_ref,
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
        Ok(())
    }
}
