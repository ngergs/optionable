#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::batch::v1::PodFailurePolicyRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::PodFailurePolicy {
    type Optioned = PodFailurePolicyAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyAc {
    type Optioned = PodFailurePolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::PodFailurePolicy {
    fn into_optioned(self) -> PodFailurePolicyAc {
        PodFailurePolicyAc {
            rules: Some(crate::OptionableConvert::into_optioned(self.rules)),
        }
    }
    fn try_from_optioned(value: PodFailurePolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            rules: crate::OptionableConvert::try_from_optioned(
                value
                    .rules
                    .ok_or(crate::Error {
                        missing_field: "rules",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodFailurePolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.rules {
            crate::OptionableConvert::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}
