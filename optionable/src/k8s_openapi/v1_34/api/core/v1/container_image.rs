pub struct ContainerImageOpt {
    pub names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub size_bytes: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_image::ContainerImage {
    type Optioned = ContainerImageOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerImageOpt {
    type Optioned = ContainerImageOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_image::ContainerImage {
    fn into_optioned(self) -> ContainerImageOpt {
        ContainerImageOpt {
            names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.names),
            size_bytes: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.size_bytes),
        }
    }
    fn try_from_optioned(
        value: ContainerImageOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.names)?,
            size_bytes: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.size_bytes)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerImageOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.names, other.names)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.size_bytes, other.size_bytes)?;
        Ok(())
    }
}
