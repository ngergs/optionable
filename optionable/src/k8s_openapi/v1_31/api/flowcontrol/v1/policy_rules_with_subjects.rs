#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRulesWithSubjectsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_rules: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::flowcontrol::v1::Subject,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::PolicyRulesWithSubjects {
    type Optioned = PolicyRulesWithSubjectsAc;
}
#[automatically_derived]
impl crate::Optionable for PolicyRulesWithSubjectsAc {
    type Optioned = PolicyRulesWithSubjectsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PolicyRulesWithSubjects {
    fn into_optioned(self) -> PolicyRulesWithSubjectsAc {
        PolicyRulesWithSubjectsAc {
            non_resource_rules: crate::OptionableConvert::into_optioned(
                self.non_resource_rules,
            ),
            resource_rules: crate::OptionableConvert::into_optioned(self.resource_rules),
            subjects: Some(crate::OptionableConvert::into_optioned(self.subjects)),
        }
    }
    fn try_from_optioned(
        value: PolicyRulesWithSubjectsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            non_resource_rules: crate::OptionableConvert::try_from_optioned(
                value.non_resource_rules,
            )?,
            resource_rules: crate::OptionableConvert::try_from_optioned(
                value.resource_rules,
            )?,
            subjects: crate::OptionableConvert::try_from_optioned(
                value
                    .subjects
                    .ok_or(crate::Error {
                        missing_field: "subjects",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PolicyRulesWithSubjectsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.non_resource_rules,
            other.non_resource_rules,
        )?;
        crate::OptionableConvert::merge(&mut self.resource_rules, other.resource_rules)?;
        if let Some(other_value) = other.subjects {
            crate::OptionableConvert::merge(&mut self.subjects, other_value)?;
        }
        Ok(())
    }
}
