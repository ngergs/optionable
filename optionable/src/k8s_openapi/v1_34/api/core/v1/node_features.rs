pub struct NodeFeaturesOpt {
    pub supplemental_groups_policy: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::node_features::NodeFeatures {
    type Optioned = NodeFeaturesOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeFeaturesOpt {
    type Optioned = NodeFeaturesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_features::NodeFeatures {
    fn into_optioned(self) -> NodeFeaturesOpt {
        NodeFeaturesOpt {
            supplemental_groups_policy: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.supplemental_groups_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeFeaturesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            supplemental_groups_policy: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.supplemental_groups_policy,
            )?,
        })
    }
    fn merge(&mut self, other: NodeFeaturesOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.supplemental_groups_policy,
            other.supplemental_groups_policy,
        )?;
        Ok(())
    }
}
