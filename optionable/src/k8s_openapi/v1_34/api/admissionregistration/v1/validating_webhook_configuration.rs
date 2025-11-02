#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingWebhookConfigurationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::ValidatingWebhook>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<ValidatingWebhookConfigurationAc>,
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
impl k8s_openapi::Resource for ValidatingWebhookConfigurationAc {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const KIND: &'static str = "ValidatingWebhookConfiguration";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "validatingwebhookconfigurations";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for ValidatingWebhookConfigurationAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
