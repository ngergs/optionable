#[derive(kube::Resource)]
#[resource(inherit = VolumeAttributesClass)]
pub struct VolumeAttributesClassAc {
    pub driver_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub parameters: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1beta1::VolumeAttributesClass {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttributesClassAc {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1beta1::VolumeAttributesClass {
    fn into_optioned(self) -> VolumeAttributesClassAc {
        VolumeAttributesClassAc {
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: self.metadata,
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(
        value: VolumeAttributesClassAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(
                value
                    .driver_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver_name",
                    })?,
            )?,
            metadata: value.metadata,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttributesClassAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver_name {
            crate::OptionableConvert::merge(&mut self.driver_name, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::storage::v1beta1::VolumeAttributesClass;
