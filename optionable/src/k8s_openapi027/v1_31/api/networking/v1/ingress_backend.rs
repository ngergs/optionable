#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressBackend describes all endpoints for a given service and port.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressBackendAc {
    /// resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a service.Name and service.Port must not be specified. This is a mutually exclusive setting with "Service".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<
        <::k8s_openapi027::api::core::v1::TypedLocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// service references a service as a backend. This is a mutually exclusive setting with "Resource".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<
        <::k8s_openapi027::api::networking::v1::IngressServiceBackend as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressBackend {
    type Optioned = IngressBackendAc;
}
#[automatically_derived]
impl crate::Optionable for IngressBackendAc {
    type Optioned = IngressBackendAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressBackend {
    fn into_optioned(self) -> IngressBackendAc {
        IngressBackendAc {
            resource: crate::OptionableConvert::into_optioned(self.resource),
            service: crate::OptionableConvert::into_optioned(self.service),
        }
    }
    fn try_from_optioned(value: IngressBackendAc) -> Result<Self, crate::Error> {
        Ok(Self {
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
        })
    }
    fn merge(&mut self, other: IngressBackendAc) -> Result<(), crate::Error> {
        if self.resource.is_none() {
            self.resource = crate::OptionableConvert::try_from_optioned(other.resource)?;
        } else if let Some(self_value) = self.resource.as_mut()
            && let Some(other_value) = other.resource
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.service.is_none() {
            self.service = crate::OptionableConvert::try_from_optioned(other.service)?;
        } else if let Some(self_value) = self.service.as_mut()
            && let Some(other_value) = other.service
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressBackend>
for IngressBackendAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressBackend,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressBackend, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressBackend,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for IngressBackendAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.resource, other.resource);
        k8s_openapi027::DeepMerge::merge_from(&mut self.service, other.service);
    }
}
