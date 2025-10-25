pub struct PolicyRulesWithSubjectsOpt {
    pub non_resource_rules: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule>,
    > as crate::Optionable>::Optioned,
    pub resource_rules: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule>,
    > as crate::Optionable>::Optioned,
    pub subjects: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::flowcontrol::v1::Subject,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::PolicyRulesWithSubjects {
    type Optioned = PolicyRulesWithSubjectsOpt;
}
#[automatically_derived]
impl crate::Optionable for PolicyRulesWithSubjectsOpt {
    type Optioned = PolicyRulesWithSubjectsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PolicyRulesWithSubjects {
    fn into_optioned(self) -> PolicyRulesWithSubjectsOpt {
        PolicyRulesWithSubjectsOpt {
            non_resource_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule>,
            > as crate::OptionableConvert>::into_optioned(self.non_resource_rules),
            resource_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule>,
            > as crate::OptionableConvert>::into_optioned(self.resource_rules),
            subjects: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::flowcontrol::v1::Subject,
                > as crate::OptionableConvert>::into_optioned(self.subjects),
            ),
        }
    }
    fn try_from_optioned(
        value: PolicyRulesWithSubjectsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule>,
            > as crate::OptionableConvert>::try_from_optioned(value.non_resource_rules)?,
            resource_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule>,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_rules)?,
            subjects: <std::vec::Vec<
                ::k8s_openapi::api::flowcontrol::v1::Subject,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .subjects
                    .ok_or(crate::optionable::Error {
                        missing_field: "subjects",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PolicyRulesWithSubjectsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule>,
        > as crate::OptionableConvert>::merge(
            &mut self.non_resource_rules,
            other.non_resource_rules,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule>,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_rules,
            other.resource_rules,
        )?;
        if let Some(other_value) = other.subjects {
            <std::vec::Vec<
                ::k8s_openapi::api::flowcontrol::v1::Subject,
            > as crate::OptionableConvert>::merge(&mut self.subjects, other_value)?;
        }
        Ok(())
    }
}
