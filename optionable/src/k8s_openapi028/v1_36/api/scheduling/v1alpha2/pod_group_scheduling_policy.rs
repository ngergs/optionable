#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupSchedulingPolicy defines the scheduling configuration for a PodGroup. Exactly one policy must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupSchedulingPolicyAc {
    /// Basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<
        <::k8s_openapi028::api::scheduling::v1alpha2::BasicSchedulingPolicy as crate::Optionable>::Optioned,
    >,
    /// Gang specifies that the pods in this group should be scheduled using all-or-nothing semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gang: Option<
        <::k8s_openapi028::api::scheduling::v1alpha2::GangSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy {
    type Optioned = PodGroupSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupSchedulingPolicyAc {
    type Optioned = PodGroupSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy {
    fn into_optioned(self) -> PodGroupSchedulingPolicyAc {
        PodGroupSchedulingPolicyAc {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            gang: crate::OptionableConvert::into_optioned(self.gang),
        }
    }
    fn try_from_optioned(
        value: PodGroupSchedulingPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            gang: crate::OptionableConvert::try_from_optioned(value.gang)?,
        })
    }
    fn merge(&mut self, other: PodGroupSchedulingPolicyAc) -> Result<(), crate::Error> {
        if self.basic.is_none() {
            self.basic = crate::OptionableConvert::try_from_optioned(other.basic)?;
        } else if let Some(self_value) = self.basic.as_mut()
            && let Some(other_value) = other.basic
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.gang.is_none() {
            self.gang = crate::OptionableConvert::try_from_optioned(other.gang)?;
        } else if let Some(self_value) = self.gang.as_mut()
            && let Some(other_value) = other.gang
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy,
> for PodGroupSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.basic, other.basic);
        k8s_openapi028::DeepMerge::merge_from(&mut self.gang, other.gang);
    }
}
