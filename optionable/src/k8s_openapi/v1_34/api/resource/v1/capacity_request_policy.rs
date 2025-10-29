pub struct CapacityRequestPolicyOpt {
    pub default: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub valid_range: <Option<
        ::k8s_openapi::api::resource::v1::CapacityRequestPolicyRange,
    > as crate::Optionable>::Optioned,
    pub valid_values: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::CapacityRequestPolicy {
    type Optioned = CapacityRequestPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyOpt {
    type Optioned = CapacityRequestPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::CapacityRequestPolicy {
    fn into_optioned(self) -> CapacityRequestPolicyOpt {
        CapacityRequestPolicyOpt {
            default: crate::OptionableConvert::into_optioned(self.default),
            valid_range: crate::OptionableConvert::into_optioned(self.valid_range),
            valid_values: crate::OptionableConvert::into_optioned(self.valid_values),
        }
    }
    fn try_from_optioned(
        value: CapacityRequestPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default: crate::OptionableConvert::try_from_optioned(value.default)?,
            valid_range: crate::OptionableConvert::try_from_optioned(value.valid_range)?,
            valid_values: crate::OptionableConvert::try_from_optioned(
                value.valid_values,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CapacityRequestPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.default, other.default)?;
        crate::OptionableConvert::merge(&mut self.valid_range, other.valid_range)?;
        crate::OptionableConvert::merge(&mut self.valid_values, other.valid_values)?;
        Ok(())
    }
}
