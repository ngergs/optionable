#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct VendorParametersAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::VendorParameters {
    type Optioned = VendorParametersAc;
}
#[automatically_derived]
impl crate::Optionable for VendorParametersAc {
    type Optioned = VendorParametersAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::VendorParameters {
    fn into_optioned(self) -> VendorParametersAc {
        VendorParametersAc {
            driver_name: crate::OptionableConvert::into_optioned(self.driver_name),
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(value: VendorParametersAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(value.driver_name)?,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(&mut self, other: VendorParametersAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.driver_name, other.driver_name)?;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
