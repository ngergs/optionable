#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedRuleWithOperationsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_versions: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for NamedRuleWithOperationsAc {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations {
    fn into_optioned(self) -> NamedRuleWithOperationsAc {
        NamedRuleWithOperationsAc {
            api_groups: self.api_groups,
            api_versions: self.api_versions,
            operations: self.operations,
            resource_names: self.resource_names,
            resources: self.resources,
            scope: self.scope,
        }
    }
    fn try_from_optioned(
        value: NamedRuleWithOperationsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            api_versions: value.api_versions,
            operations: value.operations,
            resource_names: value.resource_names,
            resources: value.resources,
            scope: value.scope,
        })
    }
    fn merge(&mut self, other: NamedRuleWithOperationsAc) -> Result<(), crate::Error> {
        self.api_groups = other.api_groups;
        self.api_versions = other.api_versions;
        self.operations = other.operations;
        self.resource_names = other.resource_names;
        self.resources = other.resources;
        self.scope = other.scope;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
> for NamedRuleWithOperationsAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
