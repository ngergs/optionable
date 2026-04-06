#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentConditionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
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
            type_: Some(self.type_),
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
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: ComponentConditionAc) -> Result<(), crate::Error> {
        self.error = other.error;
        self.message = other.message;
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
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
