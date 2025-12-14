#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeNodeResourcesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeResourcesAc {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    fn into_optioned(self) -> VolumeNodeResourcesAc {
        VolumeNodeResourcesAc {
            count: crate::OptionableConvert::into_optioned(self.count),
        }
    }
    fn try_from_optioned(value: VolumeNodeResourcesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
        })
    }
    fn merge(&mut self, other: VolumeNodeResourcesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1::VolumeNodeResources>
for VolumeNodeResourcesAc {
    fn from_optionable(
        value: ::k8s_openapi::api::storage::v1::VolumeNodeResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::storage::v1::VolumeNodeResources, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1::VolumeNodeResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
