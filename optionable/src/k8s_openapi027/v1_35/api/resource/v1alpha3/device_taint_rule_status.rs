#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintRuleStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus {
    type Optioned = DeviceTaintRuleStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleStatusAc {
    type Optioned = DeviceTaintRuleStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus {
    fn into_optioned(self) -> DeviceTaintRuleStatusAc {
        DeviceTaintRuleStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(value: DeviceTaintRuleStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(&mut self, other: DeviceTaintRuleStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus,
> for DeviceTaintRuleStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
