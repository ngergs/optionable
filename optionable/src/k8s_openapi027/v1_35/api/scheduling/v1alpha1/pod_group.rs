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
    pub name: std::string::String,
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
            name: self.name,
            policy: Some(crate::OptionableConvert::into_optioned(self.policy)),
        }
    }
    fn try_from_optioned(value: PodGroupAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
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
        self.name = other.name;
        if let Some(other_value) = other.policy {
            crate::OptionableConvert::merge(&mut self.policy, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::scheduling::v1alpha1::PodGroup {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
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
impl k8s_openapi027::DeepMerge for PodGroupAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.policy, other.policy);
    }
}
impl crate::merge::MapKeysEq for PodGroupAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
