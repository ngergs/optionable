#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerImageAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::ContainerImage {
    type Optioned = ContainerImageAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerImageAc {
    type Optioned = ContainerImageAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::ContainerImage {
    fn into_optioned(self) -> ContainerImageAc {
        ContainerImageAc {
            names: crate::OptionableConvert::into_optioned(self.names),
            size_bytes: crate::OptionableConvert::into_optioned(self.size_bytes),
        }
    }
    fn try_from_optioned(value: ContainerImageAc) -> Result<Self, crate::Error> {
        Ok(Self {
            names: crate::OptionableConvert::try_from_optioned(value.names)?,
            size_bytes: crate::OptionableConvert::try_from_optioned(value.size_bytes)?,
        })
    }
    fn merge(&mut self, other: ContainerImageAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.names, other.names)?;
        crate::OptionableConvert::merge(&mut self.size_bytes, other.size_bytes)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::ContainerImage>
for ContainerImageAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::ContainerImage) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::ContainerImage, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::ContainerImage,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
