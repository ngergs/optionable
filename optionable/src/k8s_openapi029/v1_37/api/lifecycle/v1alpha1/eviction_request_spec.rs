#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionRequestSpec is a specification of an EvictionRequest.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionRequestSpecAc {
    /// intent specifies the action that should be taken for the specified target.
    ///
    /// - Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.
    ///   If all requesters' intents are withdrawn for a common target, the eviction will be canceled.
    ///   Cancellation consequences:
    ///   - Inactive responders will never run.
    ///   - Active responders are expected to cancel the eviction.
    ///   - Completed or Interrupted responders should not take any action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<std::string::String>,
    /// requester allows you to identify the entity, that requested the eviction of the target.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field is required and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<std::string::String>,
    /// target contains a reference to an object (e.g. a pod) that should be evicted. This field is required and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec {
    type Optioned = EvictionRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionRequestSpecAc {
    type Optioned = EvictionRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec {
    fn into_optioned(self) -> EvictionRequestSpecAc {
        EvictionRequestSpecAc {
            intent: Some(self.intent),
            requester: Some(self.requester),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(value: EvictionRequestSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            intent: value
                .intent
                .ok_or(crate::Error {
                    missing_field: "intent",
                })?,
            requester: value
                .requester
                .ok_or(crate::Error {
                    missing_field: "requester",
                })?,
            target: crate::OptionableConvert::try_from_optioned(
                value
                    .target
                    .ok_or(crate::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: EvictionRequestSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.intent {
            self.intent = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.requester {
            self.requester = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec,
> for EvictionRequestSpecAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionRequestSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.intent, other.intent);
        k8s_openapi029::DeepMerge::merge_from(&mut self.requester, other.requester);
        k8s_openapi029::DeepMerge::merge_from(&mut self.target, other.target);
    }
}
