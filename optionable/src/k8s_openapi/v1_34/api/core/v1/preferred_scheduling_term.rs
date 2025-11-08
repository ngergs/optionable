#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PreferredSchedulingTermAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        <::k8s_openapi::api::core::v1::NodeSelectorTerm as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PreferredSchedulingTerm {
    type Optioned = PreferredSchedulingTermAc;
}
#[automatically_derived]
impl crate::Optionable for PreferredSchedulingTermAc {
    type Optioned = PreferredSchedulingTermAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PreferredSchedulingTerm {
    fn into_optioned(self) -> PreferredSchedulingTermAc {
        PreferredSchedulingTermAc {
            preference: Some(crate::OptionableConvert::into_optioned(self.preference)),
            weight: Some(self.weight),
        }
    }
    fn try_from_optioned(
        value: PreferredSchedulingTermAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            preference: crate::OptionableConvert::try_from_optioned(
                value
                    .preference
                    .ok_or(crate::Error {
                        missing_field: "preference",
                    })?,
            )?,
            weight: value
                .weight
                .ok_or(crate::Error {
                    missing_field: "weight",
                })?,
        })
    }
    fn merge(&mut self, other: PreferredSchedulingTermAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.preference {
            crate::OptionableConvert::merge(&mut self.preference, other_value)?;
        }
        if let Some(other_value) = other.weight {
            self.weight = other_value;
        }
        Ok(())
    }
}
