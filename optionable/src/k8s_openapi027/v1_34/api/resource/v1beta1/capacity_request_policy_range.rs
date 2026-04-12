#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CapacityRequestPolicyRange defines a valid range for consumable capacity values.
///
///   - If the requested amount is less than Min, it is rounded up to the Min value.
///   - If Step is set and the requested amount is between Min and Max but not aligned with Step,
///     it will be rounded up to the next value equal to Min + (n * Step).
///   - If Step is not set, the requested amount is used as-is if it falls within the range Min to Max (if set).
///   - If the requested or rounded amount exceeds Max (if set), the request does not satisfy the policy,
///     and the device cannot be allocated.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapacityRequestPolicyRangeAc {
    /// Max defines the upper limit for capacity that can be requested.
    ///
    /// Max must be less than or equal to the capacity value. Min and requestPolicy.default must be less than or equal to the maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Min specifies the minimum capacity allowed for a consumption request.
    ///
    /// Min must be greater than or equal to zero, and less than or equal to the capacity value. requestPolicy.default must be more than or equal to the minimum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Step defines the step size between valid capacity amounts within the range.
    ///
    /// Max (if set) and requestPolicy.default must be a multiple of Step. Min + Step must be less than or equal to the capacity value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyRangeAc {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange {
    fn into_optioned(self) -> CapacityRequestPolicyRangeAc {
        CapacityRequestPolicyRangeAc {
            max: crate::OptionableConvert::into_optioned(self.max),
            min: Some(crate::OptionableConvert::into_optioned(self.min)),
            step: crate::OptionableConvert::into_optioned(self.step),
        }
    }
    fn try_from_optioned(
        value: CapacityRequestPolicyRangeAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max: crate::OptionableConvert::try_from_optioned(value.max)?,
            min: crate::OptionableConvert::try_from_optioned(
                value
                    .min
                    .ok_or(crate::Error {
                        missing_field: "min",
                    })?,
            )?,
            step: crate::OptionableConvert::try_from_optioned(value.step)?,
        })
    }
    fn merge(
        &mut self,
        other: CapacityRequestPolicyRangeAc,
    ) -> Result<(), crate::Error> {
        if self.max.is_none() {
            self.max = other.max;
        }
        if let Some(other_value) = other.max {
            crate::OptionableConvert::merge(&mut self.max, other_value)?;
        }
        if let Some(other_value) = other.min {
            self.min = other_value;
        }
        if self.step.is_none() {
            self.step = other.step;
        }
        if let Some(other_value) = other.step {
            crate::OptionableConvert::merge(&mut self.step, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange,
> for CapacityRequestPolicyRangeAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::CapacityRequestPolicyRange,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
