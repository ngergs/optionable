#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = ServiceAccount)]
pub struct ServiceAccountAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServiceAccount {
    type Optioned = ServiceAccountAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountAc {
    type Optioned = ServiceAccountAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ServiceAccount {
    fn into_optioned(self) -> ServiceAccountAc {
        ServiceAccountAc {
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
    fn try_from_optioned(
        value: ServiceAccountAc,
    ) -> Result<Self, crate::optionable::Error> {
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
    fn merge(
        &mut self,
        other: ServiceAccountAc,
    ) -> Result<(), crate::optionable::Error> {
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
#[allow(unused_imports)]
use ::k8s_openapi::api::core::v1::ServiceAccount;
