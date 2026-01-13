#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceAccountAc {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::LocalObjectReference>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::ObjectReference>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ServiceAccount {
    type Optioned = ServiceAccountAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountAc {
    type Optioned = ServiceAccountAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ServiceAccount {
    fn into_optioned(self) -> ServiceAccountAc {
        ServiceAccountAc {
            api_version: Default::default(),
            kind: Default::default(),
            automount_service_account_token: crate::OptionableConvert::into_optioned(
                self.automount_service_account_token,
            ),
            image_pull_secrets: crate::OptionableConvert::into_optioned(
                self.image_pull_secrets,
            ),
            metadata: self.metadata,
            secrets: crate::OptionableConvert::into_optioned(self.secrets),
        }
    }
    fn try_from_optioned(value: ServiceAccountAc) -> Result<Self, crate::Error> {
        Ok(Self {
            automount_service_account_token: crate::OptionableConvert::try_from_optioned(
                value.automount_service_account_token,
            )?,
            image_pull_secrets: crate::OptionableConvert::try_from_optioned(
                value.image_pull_secrets,
            )?,
            metadata: value.metadata,
            secrets: crate::OptionableConvert::try_from_optioned(value.secrets)?,
        })
    }
    fn merge(&mut self, other: ServiceAccountAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.automount_service_account_token,
            other.automount_service_account_token,
        )?;
        crate::OptionableConvert::merge(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.secrets, other.secrets)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ServiceAccount>
for ServiceAccountAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ServiceAccount) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ServiceAccount, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ServiceAccount,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ServiceAccountAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ServiceAccountAc {
    type Ty = <k8s_openapi027::api::core::v1::ServiceAccount as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_serviceaccountac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::ServiceAccount>();
}
