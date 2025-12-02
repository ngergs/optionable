#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct DriverRequestsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::ResourceRequest>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::DriverRequests {
    type Optioned = DriverRequestsAc;
}
#[automatically_derived]
impl crate::Optionable for DriverRequestsAc {
    type Optioned = DriverRequestsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::DriverRequests {
    fn into_optioned(self) -> DriverRequestsAc {
        DriverRequestsAc {
            driver_name: crate::OptionableConvert::into_optioned(self.driver_name),
            requests: crate::OptionableConvert::into_optioned(self.requests),
            vendor_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_parameters,
            ),
        }
    }
    fn try_from_optioned(value: DriverRequestsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(value.driver_name)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
            vendor_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_parameters,
            )?,
        })
    }
    fn merge(&mut self, other: DriverRequestsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.driver_name, other.driver_name)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        crate::OptionableConvert::merge(
            &mut self.vendor_parameters,
            other.vendor_parameters,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha2::DriverRequests>
for DriverRequestsAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::DriverRequests,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1alpha2::DriverRequests, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::DriverRequests,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
