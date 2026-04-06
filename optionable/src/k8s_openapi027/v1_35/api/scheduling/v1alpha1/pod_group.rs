#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroup represents a set of pods with a common scheduling policy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupAc {
    /// Name is a unique identifier for the PodGroup within the Workload. It must be a DNS label. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Policy defines the scheduling policy for this PodGroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<
        <::k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::scheduling::v1alpha1::PodGroup {
    type Optioned = PodGroupAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupAc {
    type Optioned = PodGroupAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::scheduling::v1alpha1::PodGroup {
    fn into_optioned(self) -> PodGroupAc {
        PodGroupAc {
            name: Some(self.name),
            policy: Some(crate::OptionableConvert::into_optioned(self.policy)),
        }
    }
    fn try_from_optioned(value: PodGroupAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            policy: crate::OptionableConvert::try_from_optioned(
                value
                    .policy
                    .ok_or(crate::Error {
                        missing_field: "policy",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodGroupAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.policy {
            crate::OptionableConvert::merge(&mut self.policy, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::scheduling::v1alpha1::PodGroup>
for PodGroupAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1alpha1::PodGroup,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::scheduling::v1alpha1::PodGroup, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1alpha1::PodGroup,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
