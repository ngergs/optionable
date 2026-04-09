#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceCIDRStatusAc {
    /// conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR. Current service state
    pub conditions: Option<
        std::vec::Vec<::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::ServiceCIDRStatus {
    type Optioned = ServiceCIDRStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceCIDRStatusAc {
    type Optioned = ServiceCIDRStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::ServiceCIDRStatus {
    fn into_optioned(self) -> ServiceCIDRStatusAc {
        ServiceCIDRStatusAc {
            conditions: self.conditions,
        }
    }
    fn try_from_optioned(value: ServiceCIDRStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: value.conditions,
        })
    }
    fn merge(&mut self, other: ServiceCIDRStatusAc) -> Result<(), crate::Error> {
        self.conditions = other.conditions;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::ServiceCIDRStatus>
for ServiceCIDRStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::ServiceCIDRStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::ServiceCIDRStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::ServiceCIDRStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
