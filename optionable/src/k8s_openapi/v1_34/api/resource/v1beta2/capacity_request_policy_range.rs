#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CapacityRequestPolicyRangeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::CapacityRequestPolicyRange {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequestPolicyRangeAc {
    type Optioned = CapacityRequestPolicyRangeAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::CapacityRequestPolicyRange {
    fn into_optioned(self) -> CapacityRequestPolicyRangeAc {
        CapacityRequestPolicyRangeAc {
            max: crate::OptionableConvert::into_optioned(self.max),
            min: Some(crate::OptionableConvert::into_optioned(self.min)),
            step: crate::OptionableConvert::into_optioned(self.step),
        }
    }
    fn try_from_optioned(
        value: CapacityRequestPolicyRangeAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max: crate::OptionableConvert::try_from_optioned(value.max)?,
            min: crate::OptionableConvert::try_from_optioned(
                value
                    .min
                    .ok_or(crate::optionable::Error {
                        missing_field: "min",
                    })?,
            )?,
            step: crate::OptionableConvert::try_from_optioned(value.step)?,
        })
    }
    fn merge(
        &mut self,
        other: CapacityRequestPolicyRangeAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.max, other.max)?;
        if let Some(other_value) = other.min {
            crate::OptionableConvert::merge(&mut self.min, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.step, other.step)?;
        Ok(())
    }
}
