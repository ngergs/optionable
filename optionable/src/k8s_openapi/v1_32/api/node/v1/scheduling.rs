#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SchedulingAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::node::v1::Scheduling {
    type Optioned = SchedulingAc;
}
#[automatically_derived]
impl crate::Optionable for SchedulingAc {
    type Optioned = SchedulingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::node::v1::Scheduling {
    fn into_optioned(self) -> SchedulingAc {
        SchedulingAc {
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: SchedulingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: SchedulingAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::node::v1::Scheduling> for SchedulingAc {
    fn from_optionable(value: ::k8s_openapi::api::node::v1::Scheduling) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::node::v1::Scheduling, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::node::v1::Scheduling,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
