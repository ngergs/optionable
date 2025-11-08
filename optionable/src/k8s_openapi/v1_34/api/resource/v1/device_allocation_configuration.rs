#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAllocationConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration {
    type Optioned = DeviceAllocationConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationConfigurationAc {
    type Optioned = DeviceAllocationConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration {
    fn into_optioned(self) -> DeviceAllocationConfigurationAc {
        DeviceAllocationConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
            requests: crate::OptionableConvert::into_optioned(self.requests),
            source: Some(crate::OptionableConvert::into_optioned(self.source)),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
            source: crate::OptionableConvert::try_from_optioned(
                value
                    .source
                    .ok_or(crate::Error {
                        missing_field: "source",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationConfigurationAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        if let Some(other_value) = other.source {
            crate::OptionableConvert::merge(&mut self.source, other_value)?;
        }
        Ok(())
    }
}
