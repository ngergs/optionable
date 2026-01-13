#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GangSchedulingPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy {
    type Optioned = GangSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for GangSchedulingPolicyAc {
    type Optioned = GangSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy {
    fn into_optioned(self) -> GangSchedulingPolicyAc {
        GangSchedulingPolicyAc {
            min_count: Some(self.min_count),
        }
    }
    fn try_from_optioned(value: GangSchedulingPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_count: value
                .min_count
                .ok_or(crate::Error {
                    missing_field: "min_count",
                })?,
        })
    }
    fn merge(&mut self, other: GangSchedulingPolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.min_count {
            self.min_count = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy,
> for GangSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
