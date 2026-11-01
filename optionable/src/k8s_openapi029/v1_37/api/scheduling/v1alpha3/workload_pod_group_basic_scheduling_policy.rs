#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupBasicSchedulingPolicy indicates standard Kubernetes scheduling behavior.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupBasicSchedulingPolicyAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi029::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy {
    type Optioned = WorkloadPodGroupBasicSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupBasicSchedulingPolicyAc {
    type Optioned = WorkloadPodGroupBasicSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy {
    fn into_optioned(self) -> WorkloadPodGroupBasicSchedulingPolicyAc {
        WorkloadPodGroupBasicSchedulingPolicyAc(
            Some(crate::OptionableConvert::into_optioned(self.0)),
        )
    }
    fn try_from_optioned(
        value: WorkloadPodGroupBasicSchedulingPolicyAc,
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
        other: WorkloadPodGroupBasicSchedulingPolicyAc,
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
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy,
> for WorkloadPodGroupBasicSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupBasicSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.0, other.0);
    }
}
