pub struct PreferredSchedulingTermOpt {
    pub preference: Option<
        <::k8s_openapi::api::core::v1::NodeSelectorTerm as crate::Optionable>::Optioned,
    >,
    pub weight: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::preferred_scheduling_term::PreferredSchedulingTerm {
    type Optioned = PreferredSchedulingTermOpt;
}
#[automatically_derived]
impl crate::Optionable for PreferredSchedulingTermOpt {
    type Optioned = PreferredSchedulingTermOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::preferred_scheduling_term::PreferredSchedulingTerm {
    fn into_optioned(self) -> PreferredSchedulingTermOpt {
        PreferredSchedulingTermOpt {
            preference: Some(
                <::k8s_openapi::api::core::v1::NodeSelectorTerm as crate::OptionableConvert>::into_optioned(
                    self.preference,
                ),
            ),
            weight: Some(self.weight),
        }
    }
    fn try_from_optioned(
        value: PreferredSchedulingTermOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            preference: <::k8s_openapi::api::core::v1::NodeSelectorTerm as crate::OptionableConvert>::try_from_optioned(
                value
                    .preference
                    .ok_or(crate::optionable::Error {
                        missing_field: "preference",
                    })?,
            )?,
            weight: value
                .weight
                .ok_or(crate::optionable::Error {
                    missing_field: "weight",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PreferredSchedulingTermOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.preference {
            <::k8s_openapi::api::core::v1::NodeSelectorTerm as crate::OptionableConvert>::merge(
                &mut self.preference,
                other_value,
            )?;
        }
        if let Some(other_value) = other.weight {
            self.weight = other_value;
        }
        Ok(())
    }
}
