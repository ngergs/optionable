#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Requester allows you to identify the entity, that requested the eviction of the target.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequesterAc {
    /// intent specifies the action that should be taken for the specified target.
    ///
    /// - Eviction means that the requester is interested in the eviction of the target. - Withdrawn means that the requester is no longer interested in the eviction of the target.
    ///   If all requesters' intents are withdrawn, the eviction will be canceled.
    ///   Cancellation consequences:
    ///   - Inactive responders will never run.
    ///   - Active responders are expected to cancel the eviction.
    ///   - Completed or Interrupted responders should not take any action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<std::string::String>,
    /// name allows you to identify the entity, that requested the eviction of the target.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field must be unique for each requester. This field is required.
    pub name: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::Requester {
    type Optioned = RequesterAc;
}
#[automatically_derived]
impl crate::Optionable for RequesterAc {
    type Optioned = RequesterAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::lifecycle::v1alpha1::Requester {
    fn into_optioned(self) -> RequesterAc {
        RequesterAc {
            intent: Some(self.intent),
            name: self.name,
        }
    }
    fn try_from_optioned(value: RequesterAc) -> Result<Self, crate::Error> {
        Ok(Self {
            intent: value
                .intent
                .ok_or(crate::Error {
                    missing_field: "intent",
                })?,
            name: value.name,
        })
    }
    fn merge(&mut self, other: RequesterAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.intent {
            self.intent = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.name = other.name;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::lifecycle::v1alpha1::Requester {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::Requester>
for RequesterAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::Requester,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::lifecycle::v1alpha1::Requester, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::Requester,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for RequesterAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.intent, other.intent);
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
impl crate::merge::MapKeysEq for RequesterAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
