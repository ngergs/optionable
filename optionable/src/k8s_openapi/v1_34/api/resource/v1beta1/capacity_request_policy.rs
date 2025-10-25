pub struct CapacityRequestPolicyOpt {
    pub default: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub valid_range: <Option<
        ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicyRange,
    > as crate::Optionable>::Optioned,
    pub valid_values: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicy {
    type Optioned = CapacityRequestPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyOpt {
    type Optioned = CapacityRequestPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicy {
    fn into_optioned(self) -> CapacityRequestPolicyOpt {
        CapacityRequestPolicyOpt {
            default: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.default),
            valid_range: <Option<
                ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicyRange,
            > as crate::OptionableConvert>::into_optioned(self.valid_range),
            valid_values: <Option<
                std::vec::Vec<::k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
            > as crate::OptionableConvert>::into_optioned(self.valid_values),
        }
    }
    fn try_from_optioned(
        value: CapacityRequestPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.default)?,
            valid_range: <Option<
                ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicyRange,
            > as crate::OptionableConvert>::try_from_optioned(value.valid_range)?,
            valid_values: <Option<
                std::vec::Vec<::k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
            > as crate::OptionableConvert>::try_from_optioned(value.valid_values)?,
        })
    }
    fn merge(
        &mut self,
        other: CapacityRequestPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.default, other.default)?;
        <Option<
            ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicyRange,
        > as crate::OptionableConvert>::merge(&mut self.valid_range, other.valid_range)?;
        <Option<
            std::vec::Vec<::k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
        > as crate::OptionableConvert>::merge(
            &mut self.valid_values,
            other.valid_values,
        )?;
        Ok(())
    }
}
