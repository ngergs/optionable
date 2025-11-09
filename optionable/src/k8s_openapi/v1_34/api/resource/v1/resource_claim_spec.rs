#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1::DeviceClaim,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ResourceClaimSpec {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSpecAc {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::ResourceClaimSpec {
    fn into_optioned(self) -> ResourceClaimSpecAc {
        ResourceClaimSpecAc {
            devices: crate::OptionableConvert::into_optioned(self.devices),
        }
    }
    fn try_from_optioned(value: ResourceClaimSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
        })
    }
    fn merge(&mut self, other: ResourceClaimSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        Ok(())
    }
}
