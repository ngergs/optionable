#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositePodGroupSchedulingConstraints defines scheduling constraints (e.g. topology) for a CompositePodGroup.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositePodGroupSchedulingConstraintsAc {
    /// topology defines the topology constraints for the composite pod group. Currently only a single topology constraint can be specified. This may change in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1beta1::TopologyConstraint as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints {
    type Optioned = CompositePodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
impl crate::Optionable for CompositePodGroupSchedulingConstraintsAc {
    type Optioned = CompositePodGroupSchedulingConstraintsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints {
    fn into_optioned(self) -> CompositePodGroupSchedulingConstraintsAc {
        CompositePodGroupSchedulingConstraintsAc {
            topology: crate::OptionableConvert::into_optioned(self.topology),
        }
    }
    fn try_from_optioned(
        value: CompositePodGroupSchedulingConstraintsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            topology: crate::OptionableConvert::try_from_optioned(value.topology)?,
        })
    }
    fn merge(
        &mut self,
        other: CompositePodGroupSchedulingConstraintsAc,
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
    k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints,
> for CompositePodGroupSchedulingConstraintsAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1beta1::CompositePodGroupSchedulingConstraints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositePodGroupSchedulingConstraintsAc {
    fn merge_from(&mut self, other: Self) {
        self.topology = other.topology;
    }
}
