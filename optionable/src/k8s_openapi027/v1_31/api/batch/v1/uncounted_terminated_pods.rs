#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted in Job status counters.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UncountedTerminatedPodsAc {
    /// failed holds UIDs of failed Pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<std::vec::Vec<std::string::String>>,
    /// succeeded holds UIDs of succeeded Pods.
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
        if self.failed.is_none() {
            self.failed = crate::OptionableConvert::try_from_optioned(other.failed)?;
        } else if let Some(self_value) = self.failed.as_mut()
            && let Some(other_value) = other.failed
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        if self.succeeded.is_none() {
            self.succeeded = crate::OptionableConvert::try_from_optioned(
                other.succeeded,
            )?;
        } else if let Some(self_value) = self.succeeded.as_mut()
            && let Some(other_value) = other.succeeded
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for UncountedTerminatedPodsAc {
    fn merge_from(&mut self, other: Self) {
        crate::merge::merge_append_not_present(&mut self.failed, other.failed);
        crate::merge::merge_append_not_present(&mut self.succeeded, other.succeeded);
    }
}
