#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AllCompositeDisruptionMode means that children of a CompositePodGroup can only be disrupted or preempted together.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllCompositeDisruptionModeAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi029::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode {
    type Optioned = AllCompositeDisruptionModeAc;
}
#[automatically_derived]
impl crate::Optionable for AllCompositeDisruptionModeAc {
    type Optioned = AllCompositeDisruptionModeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode {
    fn into_optioned(self) -> AllCompositeDisruptionModeAc {
        AllCompositeDisruptionModeAc(
            Some(crate::OptionableConvert::into_optioned(self.0)),
        )
    }
    fn try_from_optioned(
        value: AllCompositeDisruptionModeAc,
    ) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(
        &mut self,
        other: AllCompositeDisruptionModeAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode,
> for AllCompositeDisruptionModeAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::AllCompositeDisruptionMode,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for AllCompositeDisruptionModeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.0, other.0);
    }
}
