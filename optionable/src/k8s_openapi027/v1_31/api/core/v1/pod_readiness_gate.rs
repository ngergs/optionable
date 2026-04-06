#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodReadinessGate contains the reference to a pod condition
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodReadinessGateAc {
    /// ConditionType refers to a condition in the pod's condition list with matching type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodReadinessGate {
    type Optioned = PodReadinessGateAc;
}
#[automatically_derived]
impl crate::Optionable for PodReadinessGateAc {
    type Optioned = PodReadinessGateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodReadinessGate {
    fn into_optioned(self) -> PodReadinessGateAc {
        PodReadinessGateAc {
            condition_type: Some(self.condition_type),
        }
    }
    fn try_from_optioned(value: PodReadinessGateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            condition_type: value
                .condition_type
                .ok_or(crate::Error {
                    missing_field: "condition_type",
                })?,
        })
    }
    fn merge(&mut self, other: PodReadinessGateAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.condition_type {
            self.condition_type = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodReadinessGate>
for PodReadinessGateAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodReadinessGate) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodReadinessGate, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodReadinessGate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
