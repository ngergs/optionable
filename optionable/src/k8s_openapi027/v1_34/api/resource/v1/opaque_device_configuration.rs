#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// OpaqueDeviceConfiguration contains configuration parameters for a driver in a format defined by the driver vendor.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OpaqueDeviceConfigurationAc {
    /// Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.
    ///
    /// An admission policy provided by the driver developer could use this to decide whether it needs to validate them.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// Parameters can contain arbitrary data. It is the responsibility of the driver developer to handle validation and versioning. Typically this includes self-identification and a version ("kind" + "apiVersion" for Kubernetes types), with conversion between different versions.
    ///
    /// The length of the raw data must be smaller or equal to 10 Ki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        <::k8s_openapi027::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for OpaqueDeviceConfigurationAc {
    type Optioned = OpaqueDeviceConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration {
    fn into_optioned(self) -> OpaqueDeviceConfigurationAc {
        OpaqueDeviceConfigurationAc {
            driver: Some(self.driver),
            parameters: Some(crate::OptionableConvert::into_optioned(self.parameters)),
        }
    }
    fn try_from_optioned(
        value: OpaqueDeviceConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
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
            self.driver = other_value;
        }
        if let Some(other_value) = other.parameters {
            self.parameters = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration>
for OpaqueDeviceConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::OpaqueDeviceConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
