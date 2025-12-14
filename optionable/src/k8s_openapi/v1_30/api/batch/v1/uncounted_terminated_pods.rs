#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UncountedTerminatedPodsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::UncountedTerminatedPods {
    type Optioned = UncountedTerminatedPodsAc;
}
#[automatically_derived]
impl crate::Optionable for UncountedTerminatedPodsAc {
    type Optioned = UncountedTerminatedPodsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::UncountedTerminatedPods {
    fn into_optioned(self) -> UncountedTerminatedPodsAc {
        UncountedTerminatedPodsAc {
            failed: crate::OptionableConvert::into_optioned(self.failed),
            succeeded: crate::OptionableConvert::into_optioned(self.succeeded),
        }
    }
    fn try_from_optioned(
        value: UncountedTerminatedPodsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            failed: crate::OptionableConvert::try_from_optioned(value.failed)?,
            succeeded: crate::OptionableConvert::try_from_optioned(value.succeeded)?,
        })
    }
    fn merge(&mut self, other: UncountedTerminatedPodsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.failed, other.failed)?;
        crate::OptionableConvert::merge(&mut self.succeeded, other.succeeded)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::batch::v1::UncountedTerminatedPods>
for UncountedTerminatedPodsAc {
    fn from_optionable(
        value: ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::batch::v1::UncountedTerminatedPods, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
