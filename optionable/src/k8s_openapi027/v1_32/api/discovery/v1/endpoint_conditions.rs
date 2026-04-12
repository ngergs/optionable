#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointConditions represents the current condition of an endpoint.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointConditionsAc {
    /// ready indicates that this endpoint is prepared to receive traffic, according to whatever system is managing the endpoint. A nil value indicates an unknown state. In most cases consumers should interpret this unknown state as ready. For compatibility reasons, ready should never be "true" for terminating endpoints, except when the normal readiness behavior is being explicitly overridden, for example when the associated Service has set the publishNotReadyAddresses flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// serving is identical to ready except that it is set regardless of the terminating state of endpoints. This condition should be set to true for a ready endpoint that is terminating. If nil, consumers should defer to the ready condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving: Option<bool>,
    /// terminating indicates that this endpoint is terminating. A nil value indicates an unknown state. Consumers should interpret this unknown state to mean that the endpoint is not terminating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::discovery::v1::EndpointConditions {
    type Optioned = EndpointConditionsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointConditionsAc {
    type Optioned = EndpointConditionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::discovery::v1::EndpointConditions {
    fn into_optioned(self) -> EndpointConditionsAc {
        EndpointConditionsAc {
            ready: self.ready,
            serving: self.serving,
            terminating: self.terminating,
        }
    }
    fn try_from_optioned(value: EndpointConditionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ready: value.ready,
            serving: value.serving,
            terminating: value.terminating,
        })
    }
    fn merge(&mut self, other: EndpointConditionsAc) -> Result<(), crate::Error> {
        if self.ready.is_none() {
            self.ready = other.ready;
        }
        if let Some(other_value) = other.ready {
            crate::OptionableConvert::merge(&mut self.ready, other_value)?;
        }
        if self.serving.is_none() {
            self.serving = other.serving;
        }
        if let Some(other_value) = other.serving {
            crate::OptionableConvert::merge(&mut self.serving, other_value)?;
        }
        if self.terminating.is_none() {
            self.terminating = other.terminating;
        }
        if let Some(other_value) = other.terminating {
            crate::OptionableConvert::merge(&mut self.terminating, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::discovery::v1::EndpointConditions>
for EndpointConditionsAc {
    fn from_optionable(
        value: k8s_openapi027::api::discovery::v1::EndpointConditions,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::discovery::v1::EndpointConditions, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::discovery::v1::EndpointConditions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
