#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
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
        self.name = other.name;
        self.namespace = other.namespace;
        self.port = other.port;
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
