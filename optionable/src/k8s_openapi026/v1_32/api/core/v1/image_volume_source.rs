#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::ImageVolumeSource {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ImageVolumeSourceAc {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::ImageVolumeSource {
    fn into_optioned(self) -> ImageVolumeSourceAc {
        ImageVolumeSourceAc {
            pull_policy: crate::OptionableConvert::into_optioned(self.pull_policy),
            reference: crate::OptionableConvert::into_optioned(self.reference),
        }
    }
    fn try_from_optioned(value: ImageVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            pull_policy: crate::OptionableConvert::try_from_optioned(value.pull_policy)?,
            reference: crate::OptionableConvert::try_from_optioned(value.reference)?,
        })
    }
    fn merge(&mut self, other: ImageVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.pull_policy, other.pull_policy)?;
        crate::OptionableConvert::merge(&mut self.reference, other.reference)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::ImageVolumeSource>
for ImageVolumeSourceAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::ImageVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::ImageVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::ImageVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
