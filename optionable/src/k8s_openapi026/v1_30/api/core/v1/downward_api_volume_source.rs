#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DownwardAPIVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::DownwardAPIVolumeFile>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::DownwardAPIVolumeSource {
    type Optioned = DownwardAPIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeSourceAc {
    type Optioned = DownwardAPIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::core::v1::DownwardAPIVolumeSource {
    fn into_optioned(self) -> DownwardAPIVolumeSourceAc {
        DownwardAPIVolumeSourceAc {
            default_mode: crate::OptionableConvert::into_optioned(self.default_mode),
            items: crate::OptionableConvert::into_optioned(self.items),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: crate::OptionableConvert::try_from_optioned(
                value.default_mode,
            )?,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
        })
    }
    fn merge(&mut self, other: DownwardAPIVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.default_mode, other.default_mode)?;
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::DownwardAPIVolumeSource>
for DownwardAPIVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::DownwardAPIVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::DownwardAPIVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::DownwardAPIVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
