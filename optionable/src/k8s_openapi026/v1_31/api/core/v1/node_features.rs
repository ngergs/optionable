#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeFeaturesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::NodeFeatures {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeFeaturesAc {
    type Optioned = NodeFeaturesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::NodeFeatures {
    fn into_optioned(self) -> NodeFeaturesAc {
        NodeFeaturesAc {
            supplemental_groups_policy: crate::OptionableConvert::into_optioned(
                self.supplemental_groups_policy,
            ),
        }
    }
    fn try_from_optioned(value: NodeFeaturesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            supplemental_groups_policy: crate::OptionableConvert::try_from_optioned(
                value.supplemental_groups_policy,
            )?,
        })
    }
    fn merge(&mut self, other: NodeFeaturesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.supplemental_groups_policy,
            other.supplemental_groups_policy,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::NodeFeatures>
for NodeFeaturesAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::NodeFeatures) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::NodeFeatures, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::NodeFeatures,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
