#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClaimAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceClaimConfiguration>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceConstraint>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequest>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceClaim {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimAc {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceClaim {
    fn into_optioned(self) -> DeviceClaimAc {
        DeviceClaimAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            constraints: crate::OptionableConvert::into_optioned(self.constraints),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: DeviceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            constraints: crate::OptionableConvert::try_from_optioned(value.constraints)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.constraints, other.constraints)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1::DeviceClaim>
for DeviceClaimAc {
    fn from_optionable(value: ::k8s_openapi::api::resource::v1::DeviceClaim) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1::DeviceClaim, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1::DeviceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
