#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NamespaceStatus is information about the current status of a Namespace.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamespaceStatusAc {
    /// Represents the latest available observations of a namespace's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NamespaceCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NamespaceStatus {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NamespaceStatusAc {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NamespaceStatus {
    fn into_optioned(self) -> NamespaceStatusAc {
        NamespaceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            phase: self.phase,
        }
    }
    fn try_from_optioned(value: NamespaceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            phase: value.phase,
        })
    }
    fn merge(&mut self, other: NamespaceStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.phase.is_none() {
            self.phase = crate::OptionableConvert::try_from_optioned(other.phase)?;
        } else if let Some(self_value) = self.phase.as_mut()
            && let Some(other_value) = other.phase
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NamespaceStatus>
for NamespaceStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NamespaceStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NamespaceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NamespaceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
