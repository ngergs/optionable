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
    pub failed: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::UncountedTerminatedPods {
    type Optioned = UncountedTerminatedPodsAc;
}
#[automatically_derived]
impl crate::Optionable for UncountedTerminatedPodsAc {
    type Optioned = UncountedTerminatedPodsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::batch::v1::UncountedTerminatedPods {
    fn into_optioned(self) -> UncountedTerminatedPodsAc {
        UncountedTerminatedPodsAc {
            failed: self.failed,
            succeeded: self.succeeded,
        }
    }
    fn try_from_optioned(
        value: UncountedTerminatedPodsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            failed: value.failed,
            succeeded: value.succeeded,
        })
    }
    fn merge(&mut self, other: UncountedTerminatedPodsAc) -> Result<(), crate::Error> {
        self.failed = other.failed;
        self.succeeded = other.succeeded;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::UncountedTerminatedPods>
for UncountedTerminatedPodsAc {
    fn from_optionable(
        value: k8s_openapi027::api::batch::v1::UncountedTerminatedPods,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::UncountedTerminatedPods, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::UncountedTerminatedPods,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
