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
    /// ready indicates that this endpoint is ready to receive traffic, according to whatever system is managing the endpoint. A nil value should be interpreted as "true". In general, an endpoint should be marked ready if it is serving and not terminating, though this can be overridden in some cases, such as when the associated Service has set the publishNotReadyAddresses flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// serving indicates that this endpoint is able to receive traffic, according to whatever system is managing the endpoint. For endpoints backed by pods, the EndpointSlice controller will mark the endpoint as serving if the pod's Ready condition is True. A nil value should be interpreted as "true".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving: Option<bool>,
    /// terminating indicates that this endpoint is terminating. A nil value should be interpreted as "false".
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
            self.ready = crate::OptionableConvert::try_from_optioned(other.ready)?;
        } else {
            crate::OptionableConvert::merge(&mut self.ready, other.ready)?;
        }
        if self.serving.is_none() {
            self.serving = crate::OptionableConvert::try_from_optioned(other.serving)?;
        } else {
            crate::OptionableConvert::merge(&mut self.serving, other.serving)?;
        }
        if self.terminating.is_none() {
            self.terminating = crate::OptionableConvert::try_from_optioned(
                other.terminating,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.terminating, other.terminating)?;
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
