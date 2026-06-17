#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeStatus represents the status of a mounted volume. At most one of its members must be specified.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeStatusAc {
    /// image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<
        <::k8s_openapi028::api::core::v1::ImageVolumeStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::core::v1::VolumeStatus {
    type Optioned = VolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeStatusAc {
    type Optioned = VolumeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::core::v1::VolumeStatus {
    fn into_optioned(self) -> VolumeStatusAc {
        VolumeStatusAc {
            image: crate::OptionableConvert::into_optioned(self.image),
        }
    }
    fn try_from_optioned(value: VolumeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            image: crate::OptionableConvert::try_from_optioned(value.image)?,
        })
    }
    fn merge(&mut self, other: VolumeStatusAc) -> Result<(), crate::Error> {
        if self.image.is_none() {
            self.image = crate::OptionableConvert::try_from_optioned(other.image)?;
        } else if let Some(self_value) = self.image.as_mut()
            && let Some(other_value) = other.image
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::core::v1::VolumeStatus>
for VolumeStatusAc {
    fn from_optionable(value: k8s_openapi028::api::core::v1::VolumeStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::core::v1::VolumeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::core::v1::VolumeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for VolumeStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.image, other.image);
    }
}
