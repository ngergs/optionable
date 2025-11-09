#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguisher_method: <Option<
        ::k8s_openapi::api::flowcontrol::v1beta3::FlowDistinguisherMethod,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_precedence: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_level_configuration: Option<
        <::k8s_openapi::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1beta3::PolicyRulesWithSubjects>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaSpec {
    type Optioned = FlowSchemaSpecAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaSpecAc {
    type Optioned = FlowSchemaSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaSpec {
    fn into_optioned(self) -> FlowSchemaSpecAc {
        FlowSchemaSpecAc {
            distinguisher_method: crate::OptionableConvert::into_optioned(
                self.distinguisher_method,
            ),
            matching_precedence: crate::OptionableConvert::into_optioned(
                self.matching_precedence,
            ),
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
            matching_precedence: crate::OptionableConvert::try_from_optioned(
                value.matching_precedence,
            )?,
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
        crate::OptionableConvert::merge(
            &mut self.distinguisher_method,
            other.distinguisher_method,
        )?;
        crate::OptionableConvert::merge(
            &mut self.matching_precedence,
            other.matching_precedence,
        )?;
        if let Some(other_value) = other.priority_level_configuration {
            crate::OptionableConvert::merge(
                &mut self.priority_level_configuration,
                other_value,
            )?;
        }
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
