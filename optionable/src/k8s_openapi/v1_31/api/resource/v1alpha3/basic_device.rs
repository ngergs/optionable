#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct BasicDeviceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1alpha3::DeviceAttribute,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::BasicDevice {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::Optionable for BasicDeviceAc {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1alpha3::BasicDevice {
    fn into_optioned(self) -> BasicDeviceAc {
        BasicDeviceAc {
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
        }
    }
    fn try_from_optioned(
        value: BasicDeviceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
        })
    }
    fn merge(&mut self, other: BasicDeviceAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        Ok(())
    }
}
