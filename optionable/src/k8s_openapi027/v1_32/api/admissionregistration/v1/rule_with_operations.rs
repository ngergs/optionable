#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOperationsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_versions: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::RuleWithOperations {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for RuleWithOperationsAc {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::RuleWithOperations {
    fn into_optioned(self) -> RuleWithOperationsAc {
        RuleWithOperationsAc {
            api_groups: self.api_groups,
            api_versions: self.api_versions,
            operations: self.operations,
            resources: self.resources,
            scope: self.scope,
        }
    }
    fn try_from_optioned(value: RuleWithOperationsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            api_versions: value.api_versions,
            operations: value.operations,
            resources: value.resources,
            scope: value.scope,
        })
    }
    fn merge(&mut self, other: RuleWithOperationsAc) -> Result<(), crate::Error> {
        self.api_groups = other.api_groups;
        self.api_versions = other.api_versions;
        self.operations = other.operations;
        self.resources = other.resources;
        self.scope = other.scope;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
> for RuleWithOperationsAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
