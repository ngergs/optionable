pub struct ResourceFieldSelectorOpt {
    pub container_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub divisor: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceFieldSelector {
    type Optioned = ResourceFieldSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceFieldSelectorOpt {
    type Optioned = ResourceFieldSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceFieldSelector {
    fn into_optioned(self) -> ResourceFieldSelectorOpt {
        ResourceFieldSelectorOpt {
            container_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.container_name),
            divisor: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.divisor),
            resource: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.resource,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceFieldSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.container_name)?,
            divisor: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.divisor)?,
            resource: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceFieldSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.container_name,
            other.container_name,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.divisor, other.divisor)?;
        if let Some(other_value) = other.resource {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.resource,
                other_value,
            )?;
        }
        Ok(())
    }
}
