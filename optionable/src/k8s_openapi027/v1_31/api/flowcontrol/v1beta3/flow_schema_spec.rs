#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaSpecAc {
    /// `distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguisher_method: Option<
        <::k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod as crate::Optionable>::Optioned,
    >,
    /// `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in \[1,10000\]. Note that if the precedence is not specified, it will be set to 1000 as default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_precedence: Option<i32>,
    /// `priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_level_configuration: Option<
        <::k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference as crate::Optionable>::Optioned,
    >,
    /// `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec {
    type Optioned = FlowSchemaSpecAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaSpecAc {
    type Optioned = FlowSchemaSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec {
    fn into_optioned(self) -> FlowSchemaSpecAc {
        FlowSchemaSpecAc {
            distinguisher_method: crate::OptionableConvert::into_optioned(
                self.distinguisher_method,
            ),
            matching_precedence: self.matching_precedence,
            priority_level_configuration: Some(
                crate::OptionableConvert::into_optioned(
                    self.priority_level_configuration,
                ),
            ),
            rules: crate::OptionableConvert::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(value: FlowSchemaSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            distinguisher_method: crate::OptionableConvert::try_from_optioned(
                value.distinguisher_method,
            )?,
            matching_precedence: value.matching_precedence,
            priority_level_configuration: crate::OptionableConvert::try_from_optioned(
                value
                    .priority_level_configuration
                    .ok_or(crate::Error {
                        missing_field: "priority_level_configuration",
                    })?,
            )?,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: FlowSchemaSpecAc) -> Result<(), crate::Error> {
        if self.distinguisher_method.is_none() {
            self.distinguisher_method = other.distinguisher_method;
        }
        if let Some(other_value) = other.distinguisher_method {
            crate::OptionableConvert::merge(
                &mut self.distinguisher_method,
                other_value,
            )?;
        }
        if self.matching_precedence.is_none() {
            self.matching_precedence = other.matching_precedence;
        }
        if let Some(other_value) = other.matching_precedence {
            crate::OptionableConvert::merge(&mut self.matching_precedence, other_value)?;
        }
        if let Some(other_value) = other.priority_level_configuration {
            self.priority_level_configuration = other_value;
        }
        if self.rules.is_none() {
            self.rules = other.rules;
        }
        if let Some(other_value) = other.rules {
            crate::OptionableConvert::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec>
for FlowSchemaSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
