#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CapacityRequirements defines the capacity requirements for a specific device request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapacityRequirementsAc {
    /// Requests represent individual device resource requests for distinct resources, all of which must be provided by the device.
    ///
    /// This value is used as an additional filtering condition against the available capacity on the device. This is semantically equivalent to a CEL selector with `device.capacity\[\<domain\>\].\<name\>.compareTo(quantity(\<request quantity\>)) \>= 0`. For example, device.capacity\['test-driver.cdi.k8s.io'\].counters.compareTo(quantity('2')) \>= 0.
    ///
    /// When a requestPolicy is defined, the requested amount is adjusted upward to the nearest valid value based on the policy. If the requested amount cannot be adjusted to a valid value—because it exceeds what the requestPolicy allows— the device is considered ineligible for allocation.
    ///
    /// For any capacity that is not explicitly requested: - If no requestPolicy is set, the default consumed capacity is equal to the full device capacity
    ///   (i.e., the whole device is claimed).
    /// - If a requestPolicy is set, the default consumed capacity is determined according to that policy.
    ///
    /// If the device allows multiple allocation, the aggregated amount across all requests must not exceed the capacity value. The consumed capacity, which may be adjusted based on the requestPolicy if defined, is recorded in the resource claim’s status.devices\[*\].consumedCapacity field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::CapacityRequirements {
    type Optioned = CapacityRequirementsAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequirementsAc {
    type Optioned = CapacityRequirementsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::CapacityRequirements {
    fn into_optioned(self) -> CapacityRequirementsAc {
        CapacityRequirementsAc {
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: CapacityRequirementsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: CapacityRequirementsAc) -> Result<(), crate::Error> {
        if self.requests.is_none() {
            self.requests = other.requests;
        }
        if let Some(other_value) = other.requests {
            crate::OptionableConvert::merge(&mut self.requests, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::CapacityRequirements>
for CapacityRequirementsAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::CapacityRequirements,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta2::CapacityRequirements,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::CapacityRequirements,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
