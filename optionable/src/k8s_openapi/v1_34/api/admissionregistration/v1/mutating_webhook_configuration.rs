#[derive(kube::Resource)]
#[resource(inherit = MutatingWebhookConfiguration)]
pub struct MutatingWebhookConfigurationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MutatingWebhook>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration {
    type Optioned = MutatingWebhookConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingWebhookConfigurationAc {
    type Optioned = MutatingWebhookConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration {
    fn into_optioned(self) -> MutatingWebhookConfigurationAc {
        MutatingWebhookConfigurationAc {
            metadata: self.metadata,
            webhooks: crate::OptionableConvert::into_optioned(self.webhooks),
        }
    }
    fn try_from_optioned(
        value: MutatingWebhookConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            webhooks: crate::OptionableConvert::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingWebhookConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration;
