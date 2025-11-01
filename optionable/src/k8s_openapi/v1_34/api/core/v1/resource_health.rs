pub struct ResourceHealthAc {
    pub health: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceHealth {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceHealthAc {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceHealth {
    fn into_optioned(self) -> ResourceHealthAc {
        ResourceHealthAc {
            health: crate::OptionableConvert::into_optioned(self.health),
            resource_id: Some(crate::OptionableConvert::into_optioned(self.resource_id)),
        }
    }
    fn try_from_optioned(
        value: ResourceHealthAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            health: crate::OptionableConvert::try_from_optioned(value.health)?,
            resource_id: crate::OptionableConvert::try_from_optioned(
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
        other: ResourceHealthAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.health, other.health)?;
        if let Some(other_value) = other.resource_id {
            crate::OptionableConvert::merge(&mut self.resource_id, other_value)?;
        }
        Ok(())
    }
}
