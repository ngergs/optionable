pub struct NodeAffinityOpt {
    pub preferred_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PreferredSchedulingTerm>,
    > as crate::Optionable>::Optioned,
    pub required_during_scheduling_ignored_during_execution: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::node_affinity::NodeAffinity {
    type Optioned = NodeAffinityOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeAffinityOpt {
    type Optioned = NodeAffinityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_affinity::NodeAffinity {
    fn into_optioned(self) -> NodeAffinityOpt {
        NodeAffinityOpt {
            preferred_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PreferredSchedulingTerm>,
            > as crate::OptionableConvert>::into_optioned(
                self.preferred_during_scheduling_ignored_during_execution,
            ),
            required_during_scheduling_ignored_during_execution: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::into_optioned(
                self.required_during_scheduling_ignored_during_execution,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeAffinityOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PreferredSchedulingTerm>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(&mut self, other: NodeAffinityOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PreferredSchedulingTerm>,
        > as crate::OptionableConvert>::merge(
            &mut self.preferred_during_scheduling_ignored_during_execution,
            other.preferred_during_scheduling_ignored_during_execution,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.required_during_scheduling_ignored_during_execution,
            other.required_during_scheduling_ignored_during_execution,
        )?;
        Ok(())
    }
}
