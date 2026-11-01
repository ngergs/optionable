#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupSchedulingPolicy defines the scheduling configuration for a PodGroup. Exactly one policy must be set. The policy is chosen at creation time by setting either the Basic or Gang field. The PodGroup may not change policy after creation. Fields within chosen policy may be updated after creation when their individual fields allow it.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupSchedulingPolicyAc {
    /// basic specifies that the pods in this group should be scheduled using standard Kubernetes scheduling behavior. Setting this field at group creation time opts this group to basic scheduling; this field cannot be changed afterward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::BasicSchedulingPolicy as crate::Optionable>::Optioned,
    >,
    /// gang specifies that the pods in this group should be scheduled using all-or-nothing semantics. Setting this field at group creation time opts this group to gang scheduling; this field cannot be set or unset afterward. The minCount field within Gang scheduling policy remains mutable after group creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gang: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::GangSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy {
    type Optioned = PodGroupSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupSchedulingPolicyAc {
    type Optioned = PodGroupSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy {
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
    k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy,
> for PodGroupSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for PodGroupSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.basic, other.basic);
        k8s_openapi029::DeepMerge::merge_from(&mut self.gang, other.gang);
    }
}
