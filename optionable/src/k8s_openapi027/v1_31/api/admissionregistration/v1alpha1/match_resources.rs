#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MatchResourcesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_resource_rules: <Option<
        std::vec::Vec<
            ::k8s_openapi027::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_selector: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: <Option<
        std::vec::Vec<
            ::k8s_openapi027::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources {
    type Optioned = MatchResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for MatchResourcesAc {
    type Optioned = MatchResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources {
    fn into_optioned(self) -> MatchResourcesAc {
        MatchResourcesAc {
            exclude_resource_rules: crate::OptionableConvert::into_optioned(
                self.exclude_resource_rules,
            ),
            match_policy: crate::OptionableConvert::into_optioned(self.match_policy),
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            object_selector: crate::OptionableConvert::into_optioned(
                self.object_selector,
            ),
            resource_rules: crate::OptionableConvert::into_optioned(self.resource_rules),
        }
    }
    fn try_from_optioned(value: MatchResourcesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exclude_resource_rules: crate::OptionableConvert::try_from_optioned(
                value.exclude_resource_rules,
            )?,
            match_policy: crate::OptionableConvert::try_from_optioned(
                value.match_policy,
            )?,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            object_selector: crate::OptionableConvert::try_from_optioned(
                value.object_selector,
            )?,
            resource_rules: crate::OptionableConvert::try_from_optioned(
                value.resource_rules,
            )?,
        })
    }
    fn merge(&mut self, other: MatchResourcesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.exclude_resource_rules,
            other.exclude_resource_rules,
        )?;
        crate::OptionableConvert::merge(&mut self.match_policy, other.match_policy)?;
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        crate::OptionableConvert::merge(
            &mut self.object_selector,
            other.object_selector,
        )?;
        crate::OptionableConvert::merge(&mut self.resource_rules, other.resource_rules)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources,
> for MatchResourcesAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
