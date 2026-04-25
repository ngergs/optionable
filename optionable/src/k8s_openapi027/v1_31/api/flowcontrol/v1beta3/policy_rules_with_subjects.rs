#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyRulesWithSubjectsAc {
    /// `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::flowcontrol::v1beta3::NonResourcePolicyRule as crate::Optionable>::Optioned,
        >,
    >,
    /// `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule as crate::Optionable>::Optioned,
        >,
    >,
    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::flowcontrol::v1beta3::Subject as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects {
    type Optioned = PolicyRulesWithSubjectsAc;
}
#[automatically_derived]
impl crate::Optionable for PolicyRulesWithSubjectsAc {
    type Optioned = PolicyRulesWithSubjectsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects {
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
        if self.non_resource_rules.is_none() {
            self.non_resource_rules = crate::OptionableConvert::try_from_optioned(
                other.non_resource_rules,
            )?;
        } else if let Some(self_value) = self.non_resource_rules.as_mut()
            && let Some(other_value) = other.non_resource_rules
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_rules.is_none() {
            self.resource_rules = crate::OptionableConvert::try_from_optioned(
                other.resource_rules,
            )?;
        } else if let Some(self_value) = self.resource_rules.as_mut()
            && let Some(other_value) = other.resource_rules
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.subjects {
            crate::OptionableConvert::merge(&mut self.subjects, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects,
> for PolicyRulesWithSubjectsAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::PolicyRulesWithSubjects,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PolicyRulesWithSubjectsAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.non_resource_rules,
            other.non_resource_rules,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.resource_rules,
            other.resource_rules,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.subjects, other.subjects);
    }
}
