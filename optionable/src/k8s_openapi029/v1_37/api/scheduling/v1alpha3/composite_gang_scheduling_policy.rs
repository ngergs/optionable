#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositeGangSchedulingPolicy indicates that the groups belonging to the composite group should be scheduled using all-or-nothing semantics.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositeGangSchedulingPolicyAc {
    /// minGroupCount is the minimum number of child groups that must be schedulable or scheduled at the same time for the scheduler to admit the entire group. It must be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_group_count: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy {
    type Optioned = CompositeGangSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for CompositeGangSchedulingPolicyAc {
    type Optioned = CompositeGangSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy {
    fn into_optioned(self) -> CompositeGangSchedulingPolicyAc {
        CompositeGangSchedulingPolicyAc {
            min_group_count: Some(self.min_group_count),
        }
    }
    fn try_from_optioned(
        value: CompositeGangSchedulingPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            min_group_count: value
                .min_group_count
                .ok_or(crate::Error {
                    missing_field: "min_group_count",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: CompositeGangSchedulingPolicyAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.min_group_count {
            self.min_group_count = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy,
> for CompositeGangSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::CompositeGangSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositeGangSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.min_group_count,
            other.min_group_count,
        );
    }
}
