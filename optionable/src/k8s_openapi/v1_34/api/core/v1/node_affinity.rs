#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAffinityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PreferredSchedulingTerm>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeAffinity {
    type Optioned = NodeAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAffinityAc {
    type Optioned = NodeAffinityAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeAffinity {
    fn into_optioned(self) -> NodeAffinityAc {
        NodeAffinityAc {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.preferred_during_scheduling_ignored_during_execution,
            ),
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.required_during_scheduling_ignored_during_execution,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeAffinityAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(&mut self, other: NodeAffinityAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.preferred_during_scheduling_ignored_during_execution,
            other.preferred_during_scheduling_ignored_during_execution,
        )?;
        crate::OptionableConvert::merge(
            &mut self.required_during_scheduling_ignored_during_execution,
            other.required_during_scheduling_ignored_during_execution,
        )?;
        Ok(())
    }
}
