#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapacityRequestPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: <Option<
        ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_range: <Option<
        ::k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: <Option<
        std::vec::Vec<::k8s_openapi026::apimachinery::pkg::api::resource::Quantity>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy {
    type Optioned = CapacityRequestPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyAc {
    type Optioned = CapacityRequestPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy {
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
        crate::OptionableConvert::merge(&mut self.default, other.default)?;
        crate::OptionableConvert::merge(&mut self.valid_range, other.valid_range)?;
        crate::OptionableConvert::merge(&mut self.valid_values, other.valid_values)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy,
> for CapacityRequestPolicyAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
