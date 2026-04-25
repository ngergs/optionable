#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Information about the condition of a component.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentConditionAc {
    /// Condition error code for a component. For example, a health check error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<std::string::String>,
    /// Message about the condition for a component. For example, information about a health check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type of condition for a component. Valid value: "Healthy"
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ComponentCondition {
    type Optioned = ComponentConditionAc;
}
#[automatically_derived]
impl crate::Optionable for ComponentConditionAc {
    type Optioned = ComponentConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ComponentCondition {
    fn into_optioned(self) -> ComponentConditionAc {
        ComponentConditionAc {
            error: self.error,
            message: self.message,
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: ComponentConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            error: value.error,
            message: value.message,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: ComponentConditionAc) -> Result<(), crate::Error> {
        if self.error.is_none() {
            self.error = crate::OptionableConvert::try_from_optioned(other.error)?;
        } else if let Some(self_value) = self.error.as_mut()
            && let Some(other_value) = other.error
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.status {
            self.status = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::core::v1::ComponentCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ComponentCondition>
for ComponentConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ComponentCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ComponentCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ComponentCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ComponentConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.error, other.error);
        k8s_openapi027::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi027::DeepMerge::merge_from(&mut self.status, other.status);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
impl crate::merge::MapKeysEq for ComponentConditionAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
    }
}
