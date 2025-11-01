pub struct PersistentVolumeClaimSpecAc {
    pub access_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub data_source: <Option<
        ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub data_source_ref: <Option<
        ::k8s_openapi::api::core::v1::TypedObjectReference,
    > as crate::Optionable>::Optioned,
    pub resources: <Option<
        ::k8s_openapi::api::core::v1::VolumeResourceRequirements,
    > as crate::Optionable>::Optioned,
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub storage_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub volume_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec {
    type Optioned = PersistentVolumeClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimSpecAc {
    type Optioned = PersistentVolumeClaimSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PersistentVolumeClaimSpec {
    fn into_optioned(self) -> PersistentVolumeClaimSpecAc {
        PersistentVolumeClaimSpecAc {
            access_modes: crate::OptionableConvert::into_optioned(self.access_modes),
            data_source: crate::OptionableConvert::into_optioned(self.data_source),
            data_source_ref: crate::OptionableConvert::into_optioned(
                self.data_source_ref,
            ),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            storage_class_name: crate::OptionableConvert::into_optioned(
                self.storage_class_name,
            ),
            volume_attributes_class_name: crate::OptionableConvert::into_optioned(
                self.volume_attributes_class_name,
            ),
            volume_mode: crate::OptionableConvert::into_optioned(self.volume_mode),
            volume_name: crate::OptionableConvert::into_optioned(self.volume_name),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            access_modes: crate::OptionableConvert::try_from_optioned(
                value.access_modes,
            )?,
            data_source: crate::OptionableConvert::try_from_optioned(value.data_source)?,
            data_source_ref: crate::OptionableConvert::try_from_optioned(
                value.data_source_ref,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            storage_class_name: crate::OptionableConvert::try_from_optioned(
                value.storage_class_name,
            )?,
            volume_attributes_class_name: crate::OptionableConvert::try_from_optioned(
                value.volume_attributes_class_name,
            )?,
            volume_mode: crate::OptionableConvert::try_from_optioned(value.volume_mode)?,
            volume_name: crate::OptionableConvert::try_from_optioned(value.volume_name)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.access_modes, other.access_modes)?;
        crate::OptionableConvert::merge(&mut self.data_source, other.data_source)?;
        crate::OptionableConvert::merge(
            &mut self.data_source_ref,
            other.data_source_ref,
        )?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        crate::OptionableConvert::merge(
            &mut self.storage_class_name,
            other.storage_class_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.volume_attributes_class_name,
            other.volume_attributes_class_name,
        )?;
        crate::OptionableConvert::merge(&mut self.volume_mode, other.volume_mode)?;
        crate::OptionableConvert::merge(&mut self.volume_name, other.volume_name)?;
        Ok(())
    }
}
