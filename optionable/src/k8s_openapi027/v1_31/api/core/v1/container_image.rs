#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Describe a container image
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerImageAc {
    /// Names by which this image is known. e.g. \["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<std::vec::Vec<std::string::String>>,
    /// The size of the image in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerImage {
    type Optioned = ContainerImageAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerImageAc {
    type Optioned = ContainerImageAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerImage {
    fn into_optioned(self) -> ContainerImageAc {
        ContainerImageAc {
            names: self.names,
            size_bytes: self.size_bytes,
        }
    }
    fn try_from_optioned(value: ContainerImageAc) -> Result<Self, crate::Error> {
        Ok(Self {
            names: value.names,
            size_bytes: value.size_bytes,
        })
    }
    fn merge(&mut self, other: ContainerImageAc) -> Result<(), crate::Error> {
        if other.names.is_some() {
            self.names = other.names;
        }
        if other.size_bytes.is_some() {
            self.size_bytes = other.size_bytes;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerImage>
for ContainerImageAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ContainerImage) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerImage, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerImage,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
