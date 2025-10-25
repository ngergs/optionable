pub struct ResourceHealthOpt {
    pub health: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::resource_health::ResourceHealth {
    type Optioned = ResourceHealthOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceHealthOpt {
    type Optioned = ResourceHealthOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::resource_health::ResourceHealth {
    fn into_optioned(self) -> ResourceHealthOpt {
        ResourceHealthOpt {
            health: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.health),
            resource_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.resource_id,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceHealthOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            health: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.health)?,
            resource_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource_id",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceHealthOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.health, other.health)?;
        if let Some(other_value) = other.resource_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.resource_id,
                other_value,
            )?;
        }
        Ok(())
    }
}
