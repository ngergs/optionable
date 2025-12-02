#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct MutatingWebhookConfigurationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MutatingWebhook>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration {
    fn into_optioned(self) -> MutatingWebhookConfigurationAc {
        MutatingWebhookConfigurationAc {
            metadata: self.metadata,
            webhooks: crate::OptionableConvert::into_optioned(self.webhooks),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: MutatingWebhookConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            webhooks: crate::OptionableConvert::try_from_optioned(value.webhooks)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingWebhookConfigurationAc,
    ) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.webhooks, other.webhooks)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration,
> for MutatingWebhookConfigurationAc {
    fn from_optionable(
        value: ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for MutatingWebhookConfigurationAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for MutatingWebhookConfigurationAc {
    type Ty = <::k8s_openapi::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
