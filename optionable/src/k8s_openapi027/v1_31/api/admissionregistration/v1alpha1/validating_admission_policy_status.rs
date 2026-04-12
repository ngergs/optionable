#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ValidatingAdmissionPolicyStatus represents the status of a ValidatingAdmissionPolicy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingAdmissionPolicyStatusAc {
    /// The conditions represent the latest available observations of a policy's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// The generation observed by the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The results of type checking for each expression. Presence of this field indicates the completion of the type checking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_checking: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus {
    type Optioned = ValidatingAdmissionPolicyStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyStatusAc {
    type Optioned = ValidatingAdmissionPolicyStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus {
    fn into_optioned(self) -> ValidatingAdmissionPolicyStatusAc {
        ValidatingAdmissionPolicyStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: self.observed_generation,
            type_checking: crate::OptionableConvert::into_optioned(self.type_checking),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: value.observed_generation,
            type_checking: crate::OptionableConvert::try_from_optioned(
                value.type_checking,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicyStatusAc,
    ) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = other.conditions;
        }
        if let Some(other_value) = other.conditions {
            crate::OptionableConvert::merge(&mut self.conditions, other_value)?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = other.observed_generation;
        }
        if let Some(other_value) = other.observed_generation {
            crate::OptionableConvert::merge(&mut self.observed_generation, other_value)?;
        }
        if self.type_checking.is_none() {
            self.type_checking = other.type_checking;
        }
        if let Some(other_value) = other.type_checking {
            crate::OptionableConvert::merge(&mut self.type_checking, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus,
> for ValidatingAdmissionPolicyStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
