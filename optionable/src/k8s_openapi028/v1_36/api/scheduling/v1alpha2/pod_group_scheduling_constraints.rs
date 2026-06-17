#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupSchedulingConstraints defines scheduling constraints (e.g. topology) for a PodGroup.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupSchedulingConstraintsAc {
    /// Topology defines the topology constraints for the pod group. Currently only a single topology constraint can be specified. This may change in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<
        std::vec::Vec<
            <::k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints {
    type Optioned = PodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupSchedulingConstraintsAc {
    type Optioned = PodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints {
    fn into_optioned(self) -> PodGroupSchedulingConstraintsAc {
        PodGroupSchedulingConstraintsAc {
            topology: crate::OptionableConvert::into_optioned(self.topology),
        }
    }
    fn try_from_optioned(
        value: PodGroupSchedulingConstraintsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            topology: crate::OptionableConvert::try_from_optioned(value.topology)?,
        })
    }
    fn merge(
        &mut self,
        other: PodGroupSchedulingConstraintsAc,
    ) -> Result<(), crate::Error> {
        if self.topology.is_none() {
            self.topology = crate::OptionableConvert::try_from_optioned(other.topology)?;
        } else if let Some(self_value) = self.topology.as_mut()
            && let Some(other_value) = other.topology
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints,
> for PodGroupSchedulingConstraintsAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupSchedulingConstraints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupSchedulingConstraintsAc {
    fn merge_from(&mut self, other: Self) {
        self.topology = other.topology;
    }
}
