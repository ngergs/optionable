pub struct DeviceClassSpecOpt {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration>,
    > as crate::Optionable>::Optioned,
    pub extended_resource_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::device_class_spec::DeviceClassSpec {
    type Optioned = DeviceClassSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassSpecOpt {
    type Optioned = DeviceClassSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::device_class_spec::DeviceClassSpec {
    fn into_optioned(self) -> DeviceClassSpecOpt {
        DeviceClassSpecOpt {
            config: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
                >,
            > as crate::OptionableConvert>::into_optioned(self.config),
            extended_resource_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.extended_resource_name),
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceSelector>,
            > as crate::OptionableConvert>::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(
        value: DeviceClassSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.config)?,
            extended_resource_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.extended_resource_name,
            )?,
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceSelector>,
            > as crate::OptionableConvert>::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceClassSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
            >,
        > as crate::OptionableConvert>::merge(&mut self.config, other.config)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.extended_resource_name,
            other.extended_resource_name,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceSelector>,
        > as crate::OptionableConvert>::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
