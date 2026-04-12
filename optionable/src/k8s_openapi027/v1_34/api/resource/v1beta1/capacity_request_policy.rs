#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CapacityRequestPolicy defines how requests consume device capacity.
///
/// Must not set more than one ValidRequestValues.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapacityRequestPolicyAc {
    /// Default specifies how much of this capacity is consumed by a request that does not contain an entry for it in DeviceRequest's Capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// ValidRange defines an acceptable quantity value range in consuming requests.
    ///
    /// If this field is set, Default must be defined and it must fall within the defined ValidRange.
    ///
    /// If the requested amount does not fall within the defined range, the request violates the policy, and this device cannot be allocated.
    ///
    /// If the request doesn't contain this capacity entry, Default value is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_range: Option<
        <::k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange as crate::Optionable>::Optioned,
    >,
    /// ValidValues defines a set of acceptable quantity values in consuming requests.
    ///
    /// Must not contain more than 10 entries. Must be sorted in ascending order.
    ///
    /// If this field is set, Default must be defined and it must be included in ValidValues list.
    ///
    /// If the requested amount does not match any valid value but smaller than some valid values, the scheduler calculates the smallest valid value that is greater than or equal to the request. That is: min(ceil(requestedValue) ∈ validValues), where requestedValue ≤ max(validValues).
    ///
    /// If the requested amount exceeds all valid values, the request violates the policy, and this device cannot be allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy {
    type Optioned = CapacityRequestPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyAc {
    type Optioned = CapacityRequestPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy {
    fn into_optioned(self) -> CapacityRequestPolicyAc {
        CapacityRequestPolicyAc {
            default: crate::OptionableConvert::into_optioned(self.default),
            valid_range: crate::OptionableConvert::into_optioned(self.valid_range),
            valid_values: crate::OptionableConvert::into_optioned(self.valid_values),
        }
    }
    fn try_from_optioned(value: CapacityRequestPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default: crate::OptionableConvert::try_from_optioned(value.default)?,
            valid_range: crate::OptionableConvert::try_from_optioned(value.valid_range)?,
            valid_values: crate::OptionableConvert::try_from_optioned(
                value.valid_values,
            )?,
        })
    }
    fn merge(&mut self, other: CapacityRequestPolicyAc) -> Result<(), crate::Error> {
        if self.default.is_none() {
            self.default = crate::OptionableConvert::try_from_optioned(other.default)?;
        } else {
            crate::OptionableConvert::merge(&mut self.default, other.default)?;
        }
        if self.valid_range.is_none() {
            self.valid_range = crate::OptionableConvert::try_from_optioned(
                other.valid_range,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.valid_range, other.valid_range)?;
        }
        if self.valid_values.is_none() {
            self.valid_values = crate::OptionableConvert::try_from_optioned(
                other.valid_values,
            )?;
        } else {
            self.valid_values = crate::OptionableConvert::try_from_optioned(
                other.valid_values,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy,
> for CapacityRequestPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
