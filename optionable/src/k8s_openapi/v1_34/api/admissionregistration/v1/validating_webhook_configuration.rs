pub struct ValidatingWebhookConfigurationOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhookConfiguration {
    type Optioned = ValidatingWebhookConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for ValidatingWebhookConfigurationOpt {
    type Optioned = ValidatingWebhookConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhookConfiguration {
    fn into_optioned(self) -> ValidatingWebhookConfigurationOpt {
        ValidatingWebhookConfigurationOpt {
            metadata: self.metadata,
            webhooks: crate::OptionableConvert::into_optioned(self.webhooks),
        }
    }
    fn try_from_optioned(
        value: ValidatingWebhookConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            webhooks: crate::OptionableConvert::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingWebhookConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
