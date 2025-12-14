#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1alpha3::DeviceClaim,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSpecAc {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec {
    fn into_optioned(self) -> ResourceClaimSpecAc {
        ResourceClaimSpecAc {
            controller: crate::OptionableConvert::into_optioned(self.controller),
            devices: crate::OptionableConvert::into_optioned(self.devices),
        }
    }
    fn try_from_optioned(value: ResourceClaimSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            controller: crate::OptionableConvert::try_from_optioned(value.controller)?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
        })
    }
    fn merge(&mut self, other: ResourceClaimSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.controller, other.controller)?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec>
for ResourceClaimSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
