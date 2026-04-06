#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressServiceBackend references a Kubernetes Service as a Backend.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressServiceBackendAc {
    /// name is the referenced service. The service must exist in the same namespace as the Ingress object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// port of the referenced service. A port name or port number is required for a IngressServiceBackend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi027::api::networking::v1::ServiceBackendPort as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressServiceBackend {
    type Optioned = IngressServiceBackendAc;
}
#[automatically_derived]
impl crate::Optionable for IngressServiceBackendAc {
    type Optioned = IngressServiceBackendAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::IngressServiceBackend {
    fn into_optioned(self) -> IngressServiceBackendAc {
        IngressServiceBackendAc {
            name: Some(self.name),
            port: crate::OptionableConvert::into_optioned(self.port),
        }
    }
    fn try_from_optioned(value: IngressServiceBackendAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
        })
    }
    fn merge(&mut self, other: IngressServiceBackendAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressServiceBackend>
for IngressServiceBackendAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressServiceBackend,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::IngressServiceBackend,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressServiceBackend,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
