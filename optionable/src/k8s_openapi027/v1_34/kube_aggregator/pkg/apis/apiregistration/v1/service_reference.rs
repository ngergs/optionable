#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceReference holds a reference to Service.legacy.k8s.io
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceReferenceAc {
    /// Name is the name of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace is the namespace of the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceReferenceAc {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference {
    fn into_optioned(self) -> ServiceReferenceAc {
        ServiceReferenceAc {
            name: self.name,
            namespace: self.namespace,
            port: self.port,
        }
    }
    fn try_from_optioned(value: ServiceReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            namespace: value.namespace,
            port: value.port,
        })
    }
    fn merge(&mut self, other: ServiceReferenceAc) -> Result<(), crate::Error> {
        if self.name.is_none() {
            self.name = other.name;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if self.namespace.is_none() {
            self.namespace = other.namespace;
        }
        if let Some(other_value) = other.namespace {
            crate::OptionableConvert::merge(&mut self.namespace, other_value)?;
        }
        if self.port.is_none() {
            self.port = other.port;
        }
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
> for ServiceReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
