pub struct ValidatingWebhookConfigurationOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
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
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            webhooks: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook,
                >,
            > as crate::OptionableConvert>::into_optioned(self.webhooks),
        }
    }
    fn try_from_optioned(
        value: ValidatingWebhookConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            webhooks: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingWebhookConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook,
            >,
        > as crate::OptionableConvert>::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
