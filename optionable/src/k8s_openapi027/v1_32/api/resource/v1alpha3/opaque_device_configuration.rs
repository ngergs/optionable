#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OpaqueDeviceConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        <::k8s_openapi027::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for OpaqueDeviceConfigurationAc {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration {
    fn into_optioned(self) -> OpaqueDeviceConfigurationAc {
        OpaqueDeviceConfigurationAc {
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            parameters: Some(crate::OptionableConvert::into_optioned(self.parameters)),
        }
    }
    fn try_from_optioned(
        value: OpaqueDeviceConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::Error {
                        missing_field: "driver",
                    })?,
            )?,
            parameters: crate::OptionableConvert::try_from_optioned(
                value
                    .parameters
                    .ok_or(crate::Error {
                        missing_field: "parameters",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: OpaqueDeviceConfigurationAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        if let Some(other_value) = other.parameters {
            crate::OptionableConvert::merge(&mut self.parameters, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration,
> for OpaqueDeviceConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
