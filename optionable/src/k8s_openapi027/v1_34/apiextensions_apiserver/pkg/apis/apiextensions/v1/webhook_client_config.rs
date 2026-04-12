#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebhookClientConfigAc {
    /// caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_bundle: Option<<::k8s_openapi027::ByteString as crate::Optionable>::Optioned>,
    /// service is a reference to the service for this webhook. Either service or url must be specified.
    ///
    /// If the webhook is running within the cluster, then you should use `service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference as crate::Optionable>::Optioned,
    >,
    /// url gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.
    ///
    /// The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.
    ///
    /// Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.
    ///
    /// The scheme must be "https"; the URL must begin with "https://".
    ///
    /// A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.
    ///
    /// Attempting to use a user or basic auth e.g. "user:password@" is not allowed. Fragments ("#...") and query parameters ("?...") are not allowed, either.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig {
    type Optioned = WebhookClientConfigAc;
}
#[automatically_derived]
impl crate::Optionable for WebhookClientConfigAc {
    type Optioned = WebhookClientConfigAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig {
    fn into_optioned(self) -> WebhookClientConfigAc {
        WebhookClientConfigAc {
            ca_bundle: crate::OptionableConvert::into_optioned(self.ca_bundle),
            service: crate::OptionableConvert::into_optioned(self.service),
            url: self.url,
        }
    }
    fn try_from_optioned(value: WebhookClientConfigAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ca_bundle: crate::OptionableConvert::try_from_optioned(value.ca_bundle)?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
            url: value.url,
        })
    }
    fn merge(&mut self, other: WebhookClientConfigAc) -> Result<(), crate::Error> {
        if self.ca_bundle.is_none() {
            self.ca_bundle = crate::OptionableConvert::try_from_optioned(
                other.ca_bundle,
            )?;
        } else if let Some(self_value) = self.ca_bundle.as_mut()
            && let Some(other_value) = other.ca_bundle
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.service.is_none() {
            self.service = crate::OptionableConvert::try_from_optioned(other.service)?;
        } else if let Some(self_value) = self.service.as_mut()
            && let Some(other_value) = other.service
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.url.is_none() {
            self.url = crate::OptionableConvert::try_from_optioned(other.url)?;
        } else if let Some(self_value) = self.url.as_mut()
            && let Some(other_value) = other.url
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
> for WebhookClientConfigAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
