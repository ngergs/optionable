pub struct PriorityClassOpt {
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub global_default: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub preemption_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub value: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::scheduling::v1::priority_class::PriorityClass {
    type Optioned = PriorityClassOpt;
}
#[automatically_derived]
impl crate::Optionable for PriorityClassOpt {
    type Optioned = PriorityClassOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::scheduling::v1::priority_class::PriorityClass {
    fn into_optioned(self) -> PriorityClassOpt {
        PriorityClassOpt {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.description),
            global_default: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.global_default),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            preemption_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.preemption_policy),
            value: Some(self.value),
        }
    }
    fn try_from_optioned(
        value: PriorityClassOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.description)?,
            global_default: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.global_default)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            preemption_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.preemption_policy)?,
            value: value
                .value
                .ok_or(crate::optionable::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityClassOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.description, other.description)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.global_default,
            other.global_default,
        )?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.preemption_policy,
            other.preemption_policy,
        )?;
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
