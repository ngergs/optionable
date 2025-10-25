pub struct StorageVersionConditionOpt {
    pub last_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub reason: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    type Optioned = StorageVersionConditionOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionConditionOpt {
    type Optioned = StorageVersionConditionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    fn into_optioned(self) -> StorageVersionConditionOpt {
        StorageVersionConditionOpt {
            last_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.last_transition_time),
            message: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.message,
                ),
            ),
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.observed_generation),
            reason: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.reason,
                ),
            ),
            status: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: StorageVersionConditionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(
                value.last_transition_time,
            )?,
            message: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .message
                    .ok_or(crate::optionable::Error {
                        missing_field: "message",
                    })?,
            )?,
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.observed_generation,
            )?,
            reason: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .reason
                    .ok_or(crate::optionable::Error {
                        missing_field: "reason",
                    })?,
            )?,
            status: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionConditionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        if let Some(other_value) = other.message {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.message,
                other_value,
            )?;
        }
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        if let Some(other_value) = other.reason {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.reason,
                other_value,
            )?;
        }
        if let Some(other_value) = other.status {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
