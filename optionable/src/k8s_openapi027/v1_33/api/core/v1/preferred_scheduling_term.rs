#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreferredSchedulingTermAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<
        <::k8s_openapi027::api::core::v1::NodeSelectorTerm as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PreferredSchedulingTerm {
    type Optioned = PreferredSchedulingTermAc;
}
#[automatically_derived]
impl crate::Optionable for PreferredSchedulingTermAc {
    type Optioned = PreferredSchedulingTermAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PreferredSchedulingTerm {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PreferredSchedulingTerm>
for PreferredSchedulingTermAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PreferredSchedulingTerm,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PreferredSchedulingTerm, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PreferredSchedulingTerm,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
