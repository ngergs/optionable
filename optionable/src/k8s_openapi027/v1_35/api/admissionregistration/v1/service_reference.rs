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
    /// `name` is the name of the service. Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// `namespace` is the namespace of the service. Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// `path` is an optional URL path which will be sent in any request to this service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::ServiceReference {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceReferenceAc {
    type Optioned = ServiceReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ServiceReference {
    fn into_optioned(self) -> ServiceReferenceAc {
        ServiceReferenceAc {
            name: Some(self.name),
            namespace: Some(self.namespace),
            path: self.path,
            port: self.port,
        }
    }
    fn try_from_optioned(value: ServiceReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            namespace: value
                .namespace
                .ok_or(crate::Error {
                    missing_field: "namespace",
                })?,
            path: value.path,
            port: value.port,
        })
    }
    fn merge(&mut self, other: ServiceReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.namespace {
            self.namespace = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.path.is_none() {
            self.path = crate::OptionableConvert::try_from_optioned(other.path)?;
        } else {
            crate::OptionableConvert::merge(&mut self.path, other.path)?;
        }
        if self.port.is_none() {
            self.port = crate::OptionableConvert::try_from_optioned(other.port)?;
        } else {
            crate::OptionableConvert::merge(&mut self.port, other.port)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::ServiceReference,
> for ServiceReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ServiceReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::ServiceReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ServiceReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
