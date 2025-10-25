pub struct WebhookClientConfigOpt {
    pub ca_bundle: <Option<::k8s_openapi::ByteString> as crate::Optionable>::Optioned,
    pub service: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ServiceReference,
    > as crate::Optionable>::Optioned,
    pub url: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig {
    type Optioned = WebhookClientConfigOpt;
}
#[automatically_derived]
impl crate::Optionable for WebhookClientConfigOpt {
    type Optioned = WebhookClientConfigOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::WebhookClientConfig {
    fn into_optioned(self) -> WebhookClientConfigOpt {
        WebhookClientConfigOpt {
            ca_bundle: <Option<
                ::k8s_openapi::ByteString,
            > as crate::OptionableConvert>::into_optioned(self.ca_bundle),
            service: <Option<
                ::k8s_openapi::api::admissionregistration::v1::ServiceReference,
            > as crate::OptionableConvert>::into_optioned(self.service),
            url: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.url),
        }
    }
    fn try_from_optioned(
        value: WebhookClientConfigOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ca_bundle: <Option<
                ::k8s_openapi::ByteString,
            > as crate::OptionableConvert>::try_from_optioned(value.ca_bundle)?,
            service: <Option<
                ::k8s_openapi::api::admissionregistration::v1::ServiceReference,
            > as crate::OptionableConvert>::try_from_optioned(value.service)?,
            url: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.url)?,
        })
    }
    fn merge(
        &mut self,
        other: WebhookClientConfigOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::ByteString,
        > as crate::OptionableConvert>::merge(&mut self.ca_bundle, other.ca_bundle)?;
        <Option<
            ::k8s_openapi::api::admissionregistration::v1::ServiceReference,
        > as crate::OptionableConvert>::merge(&mut self.service, other.service)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.url, other.url)?;
        Ok(())
    }
}
