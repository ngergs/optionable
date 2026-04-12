#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeFeaturesAc {
    /// SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeFeatures {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeFeaturesAc {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeFeatures {
    fn into_optioned(self) -> NodeFeaturesAc {
        NodeFeaturesAc {
            supplemental_groups_policy: self.supplemental_groups_policy,
        }
    }
    fn try_from_optioned(value: NodeFeaturesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            supplemental_groups_policy: value.supplemental_groups_policy,
        })
    }
    fn merge(&mut self, other: NodeFeaturesAc) -> Result<(), crate::Error> {
        if self.supplemental_groups_policy.is_none() {
            self.supplemental_groups_policy = crate::OptionableConvert::try_from_optioned(
                other.supplemental_groups_policy,
            )?;
        } else if let Some(self_value) = self.supplemental_groups_policy.as_mut()
            && let Some(other_value) = other.supplemental_groups_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeFeatures>
for NodeFeaturesAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeFeatures) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeFeatures, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeFeatures,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
