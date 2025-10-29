pub struct VolumeAttributesClassOpt {
    pub driver_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub parameters: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttributesClass {
    type Optioned = VolumeAttributesClassOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttributesClassOpt {
    type Optioned = VolumeAttributesClassOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::VolumeAttributesClass {
    fn into_optioned(self) -> VolumeAttributesClassOpt {
        VolumeAttributesClassOpt {
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(
        value: VolumeAttributesClassOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(
                value
                    .driver_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver_name",
                    })?,
            )?,
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttributesClassOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver_name {
            crate::OptionableConvert::merge(&mut self.driver_name, other_value)?;
        }
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
