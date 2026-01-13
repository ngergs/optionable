#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: <Option<
        ::k8s_openapi027::api::scheduling::v1alpha1::BasicSchedulingPolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gang: <Option<
        ::k8s_openapi027::api::scheduling::v1alpha1::GangSchedulingPolicy,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy {
    type Optioned = PodGroupPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupPolicyAc {
    type Optioned = PodGroupPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy {
    fn into_optioned(self) -> PodGroupPolicyAc {
        PodGroupPolicyAc {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            gang: crate::OptionableConvert::into_optioned(self.gang),
        }
    }
    fn try_from_optioned(value: PodGroupPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            gang: crate::OptionableConvert::try_from_optioned(value.gang)?,
        })
    }
    fn merge(&mut self, other: PodGroupPolicyAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.basic, other.basic)?;
        crate::OptionableConvert::merge(&mut self.gang, other.gang)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy>
for PodGroupPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1alpha1::PodGroupPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
