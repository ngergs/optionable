pub struct ConditionAc {
    pub last_transition_time: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    pub message: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub reason: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition {
    type Optioned = ConditionAc;
}
#[automatically_derived]
impl crate::Optionable for ConditionAc {
    type Optioned = ConditionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition {
    fn into_optioned(self) -> ConditionAc {
        ConditionAc {
            last_transition_time: Some(
                crate::OptionableConvert::into_optioned(self.last_transition_time),
            ),
            message: Some(crate::OptionableConvert::into_optioned(self.message)),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            reason: Some(crate::OptionableConvert::into_optioned(self.reason)),
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(value: ConditionAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value
                    .last_transition_time
                    .ok_or(crate::optionable::Error {
                        missing_field: "last_transition_time",
                    })?,
            )?,
            message: crate::OptionableConvert::try_from_optioned(
                value
                    .message
                    .ok_or(crate::optionable::Error {
                        missing_field: "message",
                    })?,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(
                value
                    .reason
                    .ok_or(crate::optionable::Error {
                        missing_field: "reason",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ConditionAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.last_transition_time {
            crate::OptionableConvert::merge(
                &mut self.last_transition_time,
                other_value,
            )?;
        }
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
