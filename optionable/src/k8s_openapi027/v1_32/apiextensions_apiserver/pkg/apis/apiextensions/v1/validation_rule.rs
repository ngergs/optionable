#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidationRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expression: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_old_self: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule {
    type Optioned = ValidationRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ValidationRuleAc {
    type Optioned = ValidationRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule {
    fn into_optioned(self) -> ValidationRuleAc {
        ValidationRuleAc {
            field_path: self.field_path,
            message: self.message,
            message_expression: self.message_expression,
            optional_old_self: self.optional_old_self,
            reason: self.reason,
            rule: Some(self.rule),
        }
    }
    fn try_from_optioned(value: ValidationRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_path: value.field_path,
            message: value.message,
            message_expression: value.message_expression,
            optional_old_self: value.optional_old_self,
            reason: value.reason,
            rule: value
                .rule
                .ok_or(crate::Error {
                    missing_field: "rule",
                })?,
        })
    }
    fn merge(&mut self, other: ValidationRuleAc) -> Result<(), crate::Error> {
        self.field_path = other.field_path;
        self.message = other.message;
        self.message_expression = other.message_expression;
        self.optional_old_self = other.optional_old_self;
        self.reason = other.reason;
        if let Some(other_value) = other.rule {
            self.rule = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
> for ValidationRuleAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
