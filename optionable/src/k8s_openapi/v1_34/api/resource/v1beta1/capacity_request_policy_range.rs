pub struct CapacityRequestPolicyRangeOpt {
    pub max: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub min: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    pub step: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::capacity_request_policy_range::CapacityRequestPolicyRange {
    type Optioned = CapacityRequestPolicyRangeOpt;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyRangeOpt {
    type Optioned = CapacityRequestPolicyRangeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::capacity_request_policy_range::CapacityRequestPolicyRange {
    fn into_optioned(self) -> CapacityRequestPolicyRangeOpt {
        CapacityRequestPolicyRangeOpt {
            max: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.max),
            min: Some(
                <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::into_optioned(
                    self.min,
                ),
            ),
            step: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.step),
        }
    }
    fn try_from_optioned(
        value: CapacityRequestPolicyRangeOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.max)?,
            min: <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::try_from_optioned(
                value
                    .min
                    .ok_or(crate::optionable::Error {
                        missing_field: "min",
                    })?,
            )?,
            step: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.step)?,
        })
    }
    fn merge(
        &mut self,
        other: CapacityRequestPolicyRangeOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.max, other.max)?;
        if let Some(other_value) = other.min {
            <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::merge(
                &mut self.min,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.step, other.step)?;
        Ok(())
    }
}
