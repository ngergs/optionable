pub struct ComponentStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ComponentStatus {
    type Optioned = ComponentStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ComponentStatusOpt {
    type Optioned = ComponentStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ComponentStatus {
    fn into_optioned(self) -> ComponentStatusOpt {
        ComponentStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(
        value: ComponentStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            metadata: value.metadata,
        })
    }
    fn merge(
        &mut self,
        other: ComponentStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
