#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressTLS describes the transport layer security associated with an ingress.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressTLSAc {
    /// hosts is a list of hosts included in the TLS certificate. The values in this list must match the name/s used in the tlsSecret. Defaults to the wildcard host setting for the loadbalancer controller fulfilling this Ingress, if left unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<std::vec::Vec<std::string::String>>,
    /// secretName is the name of the secret used to terminate TLS traffic on port 443. Field is left optional to allow TLS routing based on SNI hostname alone. If the SNI host in a listener conflicts with the "Host" header field used by an IngressRule, the SNI host is used for termination and value of the "Host" header is used for routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressTLS {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
impl crate::Optionable for IngressTLSAc {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressTLS {
    fn into_optioned(self) -> IngressTLSAc {
        IngressTLSAc {
            hosts: self.hosts,
            secret_name: self.secret_name,
        }
    }
    fn try_from_optioned(value: IngressTLSAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hosts: value.hosts,
            secret_name: value.secret_name,
        })
    }
    fn merge(&mut self, other: IngressTLSAc) -> Result<(), crate::Error> {
        if self.hosts.is_none() {
            self.hosts = crate::OptionableConvert::try_from_optioned(other.hosts)?;
        } else {
            self.hosts = crate::OptionableConvert::try_from_optioned(other.hosts)?;
        }
        if self.secret_name.is_none() {
            self.secret_name = crate::OptionableConvert::try_from_optioned(
                other.secret_name,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.secret_name, other.secret_name)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressTLS>
for IngressTLSAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IngressTLS) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressTLS, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressTLS,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
