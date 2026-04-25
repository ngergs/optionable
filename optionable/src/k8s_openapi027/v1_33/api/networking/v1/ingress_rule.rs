#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressRule represents the rules mapping the paths under a specified host to the related backend services. Incoming requests are first evaluated for a host match, then routed to the backend associated with the matching IngressRuleValue.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressRuleAc {
    /// host is the fully qualified domain name of a network host, as defined by RFC 3986. Note the following deviations from the "host" part of the URI as defined in RFC 3986: 1. IPs are not allowed. Currently an IngressRuleValue can only apply to
    ///    the IP in the Spec of the parent Ingress.
    /// 2. The `:` delimiter is not respected because ports are not allowed.
    ///       Currently the port of an Ingress is implicitly :80 for http and
    ///       :443 for https.
    /// Both these may change in the future. Incoming requests are matched against the host before the IngressRuleValue. If the host is unspecified, the Ingress routes all traffic based on the specified IngressRuleValue.
    ///
    /// host can be "precise" which is a domain name without the terminating dot of a network host (e.g. "foo.bar.com") or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. "*.foo.com"). The wildcard character '*' must appear by itself as the first DNS label and matches only a single label. You cannot have a wildcard label by itself (e.g. Host == "*"). Requests will be matched against the Host field in the following way: 1. If host is precise, the request matches this rule if the http host header is equal to Host. 2. If host is a wildcard, then the request matches this rule if the http host header is to equal to the suffix (removing the first label) of the wildcard rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<
        <::k8s_openapi027::api::networking::v1::HTTPIngressRuleValue as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressRule {
    type Optioned = IngressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for IngressRuleAc {
    type Optioned = IngressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressRule {
    fn into_optioned(self) -> IngressRuleAc {
        IngressRuleAc {
            host: self.host,
            http: crate::OptionableConvert::into_optioned(self.http),
        }
    }
    fn try_from_optioned(value: IngressRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: value.host,
            http: crate::OptionableConvert::try_from_optioned(value.http)?,
        })
    }
    fn merge(&mut self, other: IngressRuleAc) -> Result<(), crate::Error> {
        if self.host.is_none() {
            self.host = crate::OptionableConvert::try_from_optioned(other.host)?;
        } else if let Some(self_value) = self.host.as_mut()
            && let Some(other_value) = other.host
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.http.is_none() {
            self.http = crate::OptionableConvert::try_from_optioned(other.http)?;
        } else if let Some(self_value) = self.http.as_mut()
            && let Some(other_value) = other.http
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressRule>
for IngressRuleAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IngressRule) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for IngressRuleAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.host, other.host);
        k8s_openapi027::DeepMerge::merge_from(&mut self.http, other.http);
    }
}
