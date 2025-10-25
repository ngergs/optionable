pub struct ServiceAccountOpt {
    pub automount_service_account_token: <Option<bool> as crate::Optionable>::Optioned,
    pub image_pull_secrets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub secrets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::service_account::ServiceAccount {
    type Optioned = ServiceAccountOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountOpt {
    type Optioned = ServiceAccountOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::service_account::ServiceAccount {
    fn into_optioned(self) -> ServiceAccountOpt {
        ServiceAccountOpt {
            automount_service_account_token: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.automount_service_account_token,
            ),
            image_pull_secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
            > as crate::OptionableConvert>::into_optioned(self.image_pull_secrets),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
            > as crate::OptionableConvert>::into_optioned(self.secrets),
        }
    }
    fn try_from_optioned(
        value: ServiceAccountOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            automount_service_account_token: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.automount_service_account_token,
            )?,
            image_pull_secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
            > as crate::OptionableConvert>::try_from_optioned(value.image_pull_secrets)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
            > as crate::OptionableConvert>::try_from_optioned(value.secrets)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceAccountOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.automount_service_account_token,
            other.automount_service_account_token,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
        > as crate::OptionableConvert>::merge(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
        )?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
        > as crate::OptionableConvert>::merge(&mut self.secrets, other.secrets)?;
        Ok(())
    }
}
