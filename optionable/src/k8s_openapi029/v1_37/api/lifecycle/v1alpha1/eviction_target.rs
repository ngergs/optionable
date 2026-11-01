#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionTarget contains a reference to an object that should be evicted.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionTargetAc {
    /// pod references a pod that is subject to eviction/termination. Pods that are part of a PodGroup (.spec.schedulingGroup is set) are not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget {
    type Optioned = EvictionTargetAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionTargetAc {
    type Optioned = EvictionTargetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget {
    fn into_optioned(self) -> EvictionTargetAc {
        EvictionTargetAc {
            pod: crate::OptionableConvert::into_optioned(self.pod),
        }
    }
    fn try_from_optioned(value: EvictionTargetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            pod: crate::OptionableConvert::try_from_optioned(value.pod)?,
        })
    }
    fn merge(&mut self, other: EvictionTargetAc) -> Result<(), crate::Error> {
        if self.pod.is_none() {
            self.pod = crate::OptionableConvert::try_from_optioned(other.pod)?;
        } else if let Some(self_value) = self.pod.as_mut()
            && let Some(other_value) = other.pod
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget>
for EvictionTargetAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionTarget,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionTargetAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.pod, other.pod);
    }
}
