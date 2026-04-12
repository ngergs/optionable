#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressSpec describes the Ingress the user wishes to exist.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressSpecAc {
    /// defaultBackend is the backend that should handle requests that don't match any rule. If Rules are not specified, DefaultBackend must be specified. If DefaultBackend is not set, the handling of requests that do not match any of the rules will be up to the Ingress controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<
        <::k8s_openapi027::api::networking::v1::IngressBackend as crate::Optionable>::Optioned,
    >,
    /// ingressClassName is the name of an IngressClass cluster resource. Ingress controller implementations use this field to know whether they should be serving this Ingress resource, by a transitive connection (controller -\> IngressClass -\> Ingress resource). Although the `kubernetes.io/ingress.class` annotation (simple constant name) was never formally defined, it was widely supported by Ingress controllers to create a direct binding between Ingress controller and Ingress resources. Newly created Ingress resources should prefer using the field. However, even though the annotation is officially deprecated, for backwards compatibility reasons, ingress controllers should still honor that annotation if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<std::string::String>,
    /// rules is a list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::IngressRule as crate::Optionable>::Optioned,
        >,
    >,
    /// tls represents the TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::IngressTLS as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressSpec {
    type Optioned = IngressSpecAc;
}
#[automatically_derived]
impl crate::Optionable for IngressSpecAc {
    type Optioned = IngressSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressSpec {
    fn into_optioned(self) -> IngressSpecAc {
        IngressSpecAc {
            default_backend: crate::OptionableConvert::into_optioned(
                self.default_backend,
            ),
            ingress_class_name: self.ingress_class_name,
            rules: crate::OptionableConvert::into_optioned(self.rules),
            tls: crate::OptionableConvert::into_optioned(self.tls),
        }
    }
    fn try_from_optioned(value: IngressSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_backend: crate::OptionableConvert::try_from_optioned(
                value.default_backend,
            )?,
            ingress_class_name: value.ingress_class_name,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
            tls: crate::OptionableConvert::try_from_optioned(value.tls)?,
        })
    }
    fn merge(&mut self, other: IngressSpecAc) -> Result<(), crate::Error> {
        if self.default_backend.is_none() {
            self.default_backend = other.default_backend;
        }
        if let Some(other_value) = other.default_backend {
            crate::OptionableConvert::merge(&mut self.default_backend, other_value)?;
        }
        if self.ingress_class_name.is_none() {
            self.ingress_class_name = other.ingress_class_name;
        }
        if let Some(other_value) = other.ingress_class_name {
            crate::OptionableConvert::merge(&mut self.ingress_class_name, other_value)?;
        }
        if self.rules.is_none() {
            self.rules = other.rules;
        }
        if let Some(other_value) = other.rules {
            self.rules = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.tls.is_none() {
            self.tls = other.tls;
        }
        if let Some(other_value) = other.tls {
            self.tls = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressSpec>
for IngressSpecAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IngressSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
