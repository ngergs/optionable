#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MutatingWebhookConfiguration describes the configuration of and admission webhook that accept or reject and may change the object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingWebhookConfigurationAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Webhooks is a list of webhooks and the affected resources and operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1::MutatingWebhook as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration {
    type Optioned = MutatingWebhookConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingWebhookConfigurationAc {
    type Optioned = MutatingWebhookConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration {
    fn into_optioned(self) -> MutatingWebhookConfigurationAc {
        MutatingWebhookConfigurationAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            webhooks: crate::OptionableConvert::into_optioned(self.webhooks),
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
        if self.webhooks.is_none() {
            self.webhooks = crate::OptionableConvert::try_from_optioned(other.webhooks)?;
        } else if let Some(self_value) = self.webhooks.as_mut()
            && let Some(other_value) = other.webhooks
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration,
> for MutatingWebhookConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for MutatingWebhookConfigurationAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for MutatingWebhookConfigurationAc {
    type Ty = <k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_mutatingwebhookconfigurationac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::admissionregistration::v1::MutatingWebhookConfiguration,
    >();
}
