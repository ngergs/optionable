pub struct OpaqueDeviceConfigurationOpt {
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub parameters: Option<
        <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration {
    type Optioned = OpaqueDeviceConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for OpaqueDeviceConfigurationOpt {
    type Optioned = OpaqueDeviceConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration {
    fn into_optioned(self) -> OpaqueDeviceConfigurationOpt {
        OpaqueDeviceConfigurationOpt {
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            parameters: Some(
                <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::OptionableConvert>::into_optioned(
                    self.parameters,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: OpaqueDeviceConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            parameters: <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::OptionableConvert>::try_from_optioned(
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
        other: OpaqueDeviceConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.driver,
                other_value,
            )?;
        }
        if let Some(other_value) = other.parameters {
            <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::OptionableConvert>::merge(
                &mut self.parameters,
                other_value,
            )?;
        }
        Ok(())
    }
}
