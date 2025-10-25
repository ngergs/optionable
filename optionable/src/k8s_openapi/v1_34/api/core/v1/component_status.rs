pub struct ComponentStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
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
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ComponentStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ComponentStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        Ok(())
    }
}
