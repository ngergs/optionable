pub struct MutatingWebhookConfigurationOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MutatingWebhook>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration {
    type Optioned = MutatingWebhookConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingWebhookConfigurationOpt {
    type Optioned = MutatingWebhookConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration {
    fn into_optioned(self) -> MutatingWebhookConfigurationOpt {
        MutatingWebhookConfigurationOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            webhooks: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::MutatingWebhook,
                >,
            > as crate::OptionableConvert>::into_optioned(self.webhooks),
        }
    }
    fn try_from_optioned(
        value: MutatingWebhookConfigurationOpt,
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
                    ::k8s_openapi::api::admissionregistration::v1::MutatingWebhook,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingWebhookConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MutatingWebhook>,
        > as crate::OptionableConvert>::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
