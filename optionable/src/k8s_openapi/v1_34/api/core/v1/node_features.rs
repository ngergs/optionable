#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NodeFeaturesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeFeatures {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeFeaturesAc {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeFeatures {
    fn into_optioned(self) -> NodeFeaturesAc {
        NodeFeaturesAc {
            supplemental_groups_policy: crate::OptionableConvert::into_optioned(
                self.supplemental_groups_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeFeaturesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            supplemental_groups_policy: crate::OptionableConvert::try_from_optioned(
                value.supplemental_groups_policy,
            )?,
        })
    }
    fn merge(&mut self, other: NodeFeaturesAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.supplemental_groups_policy,
            other.supplemental_groups_policy,
        )?;
        Ok(())
    }
}
