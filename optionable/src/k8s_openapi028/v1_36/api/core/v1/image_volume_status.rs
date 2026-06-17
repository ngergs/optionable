#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ImageVolumeStatus represents the image-based volume status.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageVolumeStatusAc {
    /// ImageRef is the digest of the image used for this volume. It should have a value that's similar to the pod's status.containerStatuses\[i\].imageID. The ImageRef length should not exceed 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::core::v1::ImageVolumeStatus {
    type Optioned = ImageVolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ImageVolumeStatusAc {
    type Optioned = ImageVolumeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::core::v1::ImageVolumeStatus {
    fn into_optioned(self) -> ImageVolumeStatusAc {
        ImageVolumeStatusAc {
            image_ref: Some(self.image_ref),
        }
    }
    fn try_from_optioned(value: ImageVolumeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            image_ref: value
                .image_ref
                .ok_or(crate::Error {
                    missing_field: "image_ref",
                })?,
        })
    }
    fn merge(&mut self, other: ImageVolumeStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.image_ref {
            self.image_ref = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::core::v1::ImageVolumeStatus>
for ImageVolumeStatusAc {
    fn from_optionable(value: k8s_openapi028::api::core::v1::ImageVolumeStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::core::v1::ImageVolumeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::core::v1::ImageVolumeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for ImageVolumeStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.image_ref, other.image_ref);
    }
}
