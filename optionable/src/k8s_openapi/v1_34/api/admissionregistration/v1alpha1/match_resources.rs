pub struct MatchResourcesOpt {
    pub exclude_resource_rules: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
        >,
    > as crate::Optionable>::Optioned,
    pub match_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub object_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub resource_rules: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources {
    type Optioned = MatchResourcesOpt;
}
#[automatically_derived]
impl crate::Optionable for MatchResourcesOpt {
    type Optioned = MatchResourcesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources {
    fn into_optioned(self) -> MatchResourcesOpt {
        MatchResourcesOpt {
            exclude_resource_rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
                >,
            > as crate::OptionableConvert>::into_optioned(self.exclude_resource_rules),
            match_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.match_policy),
            namespace_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.namespace_selector),
            object_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.object_selector),
            resource_rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
                >,
            > as crate::OptionableConvert>::into_optioned(self.resource_rules),
        }
    }
    fn try_from_optioned(
        value: MatchResourcesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exclude_resource_rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.exclude_resource_rules,
            )?,
            match_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.match_policy)?,
            namespace_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace_selector)?,
            object_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.object_selector)?,
            resource_rules: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_rules)?,
        })
    }
    fn merge(
        &mut self,
        other: MatchResourcesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.exclude_resource_rules,
            other.exclude_resource_rules,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.match_policy,
            other.match_policy,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.object_selector,
            other.object_selector,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_rules,
            other.resource_rules,
        )?;
        Ok(())
    }
}
