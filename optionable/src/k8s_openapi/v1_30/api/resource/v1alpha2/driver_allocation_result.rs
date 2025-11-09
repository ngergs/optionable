#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DriverAllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_resources: <Option<
        ::k8s_openapi::api::resource::v1alpha2::NamedResourcesAllocationResult,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_request_parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::DriverAllocationResult {
    type Optioned = DriverAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DriverAllocationResultAc {
    type Optioned = DriverAllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::DriverAllocationResult {
    fn into_optioned(self) -> DriverAllocationResultAc {
        DriverAllocationResultAc {
            named_resources: crate::OptionableConvert::into_optioned(
                self.named_resources,
            ),
            vendor_request_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_request_parameters,
            ),
        }
    }
    fn try_from_optioned(value: DriverAllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            named_resources: crate::OptionableConvert::try_from_optioned(
                value.named_resources,
            )?,
            vendor_request_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_request_parameters,
            )?,
        })
    }
    fn merge(&mut self, other: DriverAllocationResultAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.named_resources,
            other.named_resources,
        )?;
        crate::OptionableConvert::merge(
            &mut self.vendor_request_parameters,
            other.vendor_request_parameters,
        )?;
        Ok(())
    }
}
