#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DownwardAPIProjectionAc {
    /// Items is a list of DownwardAPIVolume file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::DownwardAPIVolumeFile as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::DownwardAPIProjection {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIProjectionAc {
    type Optioned = DownwardAPIProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::DownwardAPIProjection {
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
        if self.items.is_none() {
            self.items = crate::OptionableConvert::try_from_optioned(other.items)?;
        } else if let Some(self_value) = self.items.as_mut()
            && let Some(other_value) = other.items
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::DownwardAPIProjection>
for DownwardAPIProjectionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::DownwardAPIProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::DownwardAPIProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::DownwardAPIProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DownwardAPIProjectionAc {
    fn merge_from(&mut self, other: Self) {
        self.items = other.items;
    }
}
