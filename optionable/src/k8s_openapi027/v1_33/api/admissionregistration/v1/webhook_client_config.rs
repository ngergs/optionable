#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebhookClientConfigAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_bundle: <Option<::k8s_openapi027::ByteString> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: <Option<
        ::k8s_openapi027::api::admissionregistration::v1::ServiceReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig {
    type Optioned = WebhookClientConfigAc;
}
#[automatically_derived]
impl crate::Optionable for WebhookClientConfigAc {
    type Optioned = WebhookClientConfigAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig {
    fn into_optioned(self) -> WebhookClientConfigAc {
        WebhookClientConfigAc {
            ca_bundle: crate::OptionableConvert::into_optioned(self.ca_bundle),
            service: crate::OptionableConvert::into_optioned(self.service),
            url: crate::OptionableConvert::into_optioned(self.url),
        }
    }
    fn try_from_optioned(value: WebhookClientConfigAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ca_bundle: crate::OptionableConvert::try_from_optioned(value.ca_bundle)?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
            url: crate::OptionableConvert::try_from_optioned(value.url)?,
        })
    }
    fn merge(&mut self, other: WebhookClientConfigAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.ca_bundle, other.ca_bundle)?;
        crate::OptionableConvert::merge(&mut self.service, other.service)?;
        crate::OptionableConvert::merge(&mut self.url, other.url)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig,
> for WebhookClientConfigAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
