#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapacityRequestPolicyRangeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: <Option<
        ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<
        <::k8s_openapi026::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: <Option<
        ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyRangeAc {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange {
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
        crate::OptionableConvert::merge(&mut self.max, other.max)?;
        if let Some(other_value) = other.min {
            crate::OptionableConvert::merge(&mut self.min, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.step, other.step)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange,
> for CapacityRequestPolicyRangeAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1beta1::CapacityRequestPolicyRange,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
