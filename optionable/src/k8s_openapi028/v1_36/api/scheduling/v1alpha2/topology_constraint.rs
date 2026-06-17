#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TopologyConstraint defines a topology constraint for a PodGroup.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopologyConstraintAc {
    /// Key specifies the key of the node label representing the topology domain. All pods within the PodGroup must be colocated within the same domain instance. Different PodGroups can land on different domain instances even if they derive from the same PodGroupTemplate. Examples: "topology.kubernetes.io/rack"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint {
    type Optioned = TopologyConstraintAc;
}
#[automatically_derived]
impl crate::Optionable for TopologyConstraintAc {
    type Optioned = TopologyConstraintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint {
    fn into_optioned(self) -> TopologyConstraintAc {
        TopologyConstraintAc {
            key: Some(self.key),
        }
    }
    fn try_from_optioned(value: TopologyConstraintAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
        })
    }
    fn merge(&mut self, other: TopologyConstraintAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint,
> for TopologyConstraintAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::TopologyConstraint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for TopologyConstraintAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.key, other.key);
    }
}
