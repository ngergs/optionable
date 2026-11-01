#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionResponder allows you to specify the responder reacting to an Eviction. Responders should observe and communicate through the Eviction Resource API to help with the graceful eviction of a target (e.g. termination of a pod).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionResponderAc {
    /// name allows you to identify the responder responding to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). Domain names *.k8s.io and *.kubernetes.io are reserved. This field must be unique for each responder. This field is required.
    pub name: std::string::String,
    /// priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.
    ///
    /// The responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.
    ///
    /// The minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::EvictionResponder {
    type Optioned = EvictionResponderAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionResponderAc {
    type Optioned = EvictionResponderAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::EvictionResponder {
    fn into_optioned(self) -> EvictionResponderAc {
        EvictionResponderAc {
            name: self.name,
            priority: Some(self.priority),
        }
    }
    fn try_from_optioned(value: EvictionResponderAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            priority: value
                .priority
                .ok_or(crate::Error {
                    missing_field: "priority",
                })?,
        })
    }
    fn merge(&mut self, other: EvictionResponderAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if let Some(other_value) = other.priority {
            self.priority = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::core::v1::EvictionResponder {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::EvictionResponder>
for EvictionResponderAc {
    fn from_optionable(value: k8s_openapi029::api::core::v1::EvictionResponder) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::EvictionResponder, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::EvictionResponder,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionResponderAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.priority, other.priority);
    }
}
impl crate::merge::MapKeysEq for EvictionResponderAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
