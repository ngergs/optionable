#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DownwardAPIProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::DownwardAPIVolumeFile>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::DownwardAPIProjection {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIProjectionAc {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::DownwardAPIProjection {
    fn into_optioned(self) -> DownwardAPIProjectionAc {
        DownwardAPIProjectionAc {
            items: crate::OptionableConvert::into_optioned(self.items),
        }
    }
    fn try_from_optioned(value: DownwardAPIProjectionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
        })
    }
    fn merge(&mut self, other: DownwardAPIProjectionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::DownwardAPIProjection>
for DownwardAPIProjectionAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::DownwardAPIProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::DownwardAPIProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::DownwardAPIProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
