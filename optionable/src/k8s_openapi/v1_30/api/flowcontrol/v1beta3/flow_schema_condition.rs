#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaConditionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaConditionAc {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition {
    fn into_optioned(self) -> FlowSchemaConditionAc {
        FlowSchemaConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            status: crate::OptionableConvert::into_optioned(self.status),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: FlowSchemaConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: FlowSchemaConditionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition,
> for FlowSchemaConditionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
