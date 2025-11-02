#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expression: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_old_self: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule {
    type Optioned = ValidationRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ValidationRuleAc {
    type Optioned = ValidationRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule {
    fn into_optioned(self) -> ValidationRuleAc {
        ValidationRuleAc {
            field_path: crate::OptionableConvert::into_optioned(self.field_path),
            message: crate::OptionableConvert::into_optioned(self.message),
            message_expression: crate::OptionableConvert::into_optioned(
                self.message_expression,
            ),
            optional_old_self: crate::OptionableConvert::into_optioned(
                self.optional_old_self,
            ),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            rule: Some(crate::OptionableConvert::into_optioned(self.rule)),
        }
    }
    fn try_from_optioned(
        value: ValidationRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field_path: crate::OptionableConvert::try_from_optioned(value.field_path)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            message_expression: crate::OptionableConvert::try_from_optioned(
                value.message_expression,
            )?,
            optional_old_self: crate::OptionableConvert::try_from_optioned(
                value.optional_old_self,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            rule: crate::OptionableConvert::try_from_optioned(
                value
                    .rule
                    .ok_or(crate::optionable::Error {
                        missing_field: "rule",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ValidationRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.field_path, other.field_path)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(
            &mut self.message_expression,
            other.message_expression,
        )?;
        crate::OptionableConvert::merge(
            &mut self.optional_old_self,
            other.optional_old_self,
        )?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        if let Some(other_value) = other.rule {
            crate::OptionableConvert::merge(&mut self.rule, other_value)?;
        }
        Ok(())
    }
}
