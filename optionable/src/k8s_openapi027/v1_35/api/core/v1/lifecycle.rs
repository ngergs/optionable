#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LifecycleAc {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_start: Option<
        <::k8s_openapi027::api::core::v1::LifecycleHandler as crate::Optionable>::Optioned,
    >,
    /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<
        <::k8s_openapi027::api::core::v1::LifecycleHandler as crate::Optionable>::Optioned,
    >,
    /// StopSignal defines which signal will be sent to a container when it is being stopped. If not specified, the default is defined by the container runtime in use. StopSignal can only be set for Pods with a non-empty .spec.os.name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Lifecycle {
    type Optioned = LifecycleAc;
}
#[automatically_derived]
impl crate::Optionable for LifecycleAc {
    type Optioned = LifecycleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Lifecycle {
    fn into_optioned(self) -> LifecycleAc {
        LifecycleAc {
            post_start: crate::OptionableConvert::into_optioned(self.post_start),
            pre_stop: crate::OptionableConvert::into_optioned(self.pre_stop),
            stop_signal: self.stop_signal,
        }
    }
    fn try_from_optioned(value: LifecycleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            post_start: crate::OptionableConvert::try_from_optioned(value.post_start)?,
            pre_stop: crate::OptionableConvert::try_from_optioned(value.pre_stop)?,
            stop_signal: value.stop_signal,
        })
    }
    fn merge(&mut self, other: LifecycleAc) -> Result<(), crate::Error> {
        if self.post_start.is_none() {
            self.post_start = crate::OptionableConvert::try_from_optioned(
                other.post_start,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.post_start, other.post_start)?;
        }
        if self.pre_stop.is_none() {
            self.pre_stop = crate::OptionableConvert::try_from_optioned(other.pre_stop)?;
        } else {
            crate::OptionableConvert::merge(&mut self.pre_stop, other.pre_stop)?;
        }
        if self.stop_signal.is_none() {
            self.stop_signal = crate::OptionableConvert::try_from_optioned(
                other.stop_signal,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.stop_signal, other.stop_signal)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Lifecycle> for LifecycleAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Lifecycle) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Lifecycle, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Lifecycle,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
