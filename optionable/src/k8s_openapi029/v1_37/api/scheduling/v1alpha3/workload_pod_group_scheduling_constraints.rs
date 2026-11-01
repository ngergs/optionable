#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupSchedulingConstraints defines leaf-level scheduling constraints, such as topology.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupSchedulingConstraintsAc {
    /// topology specifies desired topological placements for all pods within the pod group. If unset, no topology placement is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::TopologyConstraint as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints {
    type Optioned = WorkloadPodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupSchedulingConstraintsAc {
    type Optioned = WorkloadPodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints {
    fn into_optioned(self) -> WorkloadPodGroupSchedulingConstraintsAc {
        WorkloadPodGroupSchedulingConstraintsAc {
            topology: crate::OptionableConvert::into_optioned(self.topology),
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupSchedulingConstraintsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            topology: crate::OptionableConvert::try_from_optioned(value.topology)?,
        })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupSchedulingConstraintsAc,
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
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints,
> for WorkloadPodGroupSchedulingConstraintsAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupSchedulingConstraintsAc {
    fn merge_from(&mut self, other: Self) {
        self.topology = other.topology;
    }
}
