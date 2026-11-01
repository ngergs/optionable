#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositeBasicSchedulingPolicy indicates that the groups belonging to the composite group should be scheduled independently.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositeBasicSchedulingPolicyAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi029::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy {
    type Optioned = CompositeBasicSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for CompositeBasicSchedulingPolicyAc {
    type Optioned = CompositeBasicSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy {
    fn into_optioned(self) -> CompositeBasicSchedulingPolicyAc {
        CompositeBasicSchedulingPolicyAc(
            Some(crate::OptionableConvert::into_optioned(self.0)),
        )
    }
    fn try_from_optioned(
        value: CompositeBasicSchedulingPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(
        &mut self,
        other: CompositeBasicSchedulingPolicyAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy,
> for CompositeBasicSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1beta1::CompositeBasicSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositeBasicSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.0, other.0);
    }
}
