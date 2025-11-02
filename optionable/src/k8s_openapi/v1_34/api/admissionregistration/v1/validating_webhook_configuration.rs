#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = ValidatingWebhookConfiguration)]
pub struct ValidatingWebhookConfigurationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhookConfiguration {
    type Optioned = ValidatingWebhookConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingWebhookConfigurationAc {
    type Optioned = ValidatingWebhookConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhookConfiguration {
    fn into_optioned(self) -> ValidatingWebhookConfigurationAc {
        ValidatingWebhookConfigurationAc {
            metadata: self.metadata,
            webhooks: crate::OptionableConvert::into_optioned(self.webhooks),
        }
    }
    fn try_from_optioned(
        value: ValidatingWebhookConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            webhooks: crate::OptionableConvert::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingWebhookConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhookConfiguration;
