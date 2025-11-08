#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceClassConfiguration>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_resource_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::DeviceClassSpec {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassSpecAc {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceClassSpec {
    fn into_optioned(self) -> DeviceClassSpecAc {
        DeviceClassSpecAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            extended_resource_name: crate::OptionableConvert::into_optioned(
                self.extended_resource_name,
            ),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceClassSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            extended_resource_name: crate::OptionableConvert::try_from_optioned(
                value.extended_resource_name,
            )?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(&mut self, other: DeviceClassSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(
            &mut self.extended_resource_name,
            other.extended_resource_name,
        )?;
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
