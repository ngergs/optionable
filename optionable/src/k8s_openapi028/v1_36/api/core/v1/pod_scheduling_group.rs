#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodSchedulingGroup identifies the runtime scheduling group instance that a Pod belongs to. The scheduler uses this information to apply workload-aware scheduling semantics. Exactly one field must be specified.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSchedulingGroupAc {
    /// PodGroupName specifies the name of the standalone PodGroup object that represents the runtime instance of this group. Must be a DNS subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::core::v1::PodSchedulingGroup {
    type Optioned = PodSchedulingGroupAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingGroupAc {
    type Optioned = PodSchedulingGroupAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::core::v1::PodSchedulingGroup {
    fn into_optioned(self) -> PodSchedulingGroupAc {
        PodSchedulingGroupAc {
            pod_group_name: self.pod_group_name,
        }
    }
    fn try_from_optioned(value: PodSchedulingGroupAc) -> Result<Self, crate::Error> {
        Ok(Self {
            pod_group_name: value.pod_group_name,
        })
    }
    fn merge(&mut self, other: PodSchedulingGroupAc) -> Result<(), crate::Error> {
        if self.pod_group_name.is_none() {
            self.pod_group_name = crate::OptionableConvert::try_from_optioned(
                other.pod_group_name,
            )?;
        } else if let Some(self_value) = self.pod_group_name.as_mut()
            && let Some(other_value) = other.pod_group_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::core::v1::PodSchedulingGroup>
for PodSchedulingGroupAc {
    fn from_optionable(
        value: k8s_openapi028::api::core::v1::PodSchedulingGroup,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::core::v1::PodSchedulingGroup, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::core::v1::PodSchedulingGroup,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodSchedulingGroupAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.pod_group_name,
            other.pod_group_name,
        );
    }
}
