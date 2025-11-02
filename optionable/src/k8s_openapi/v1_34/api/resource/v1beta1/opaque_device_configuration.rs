#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OpaqueDeviceConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::OpaqueDeviceConfiguration {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for OpaqueDeviceConfigurationAc {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::OpaqueDeviceConfiguration {
    fn into_optioned(self) -> OpaqueDeviceConfigurationAc {
        OpaqueDeviceConfigurationAc {
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            parameters: Some(crate::OptionableConvert::into_optioned(self.parameters)),
        }
    }
    fn try_from_optioned(
        value: OpaqueDeviceConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            parameters: crate::OptionableConvert::try_from_optioned(
                value
                    .parameters
                    .ok_or(crate::optionable::Error {
                        missing_field: "parameters",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: OpaqueDeviceConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        if let Some(other_value) = other.parameters {
            crate::OptionableConvert::merge(&mut self.parameters, other_value)?;
        }
        Ok(())
    }
}
