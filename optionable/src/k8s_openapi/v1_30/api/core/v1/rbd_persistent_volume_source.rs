#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct RBDPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyring: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for RBDPersistentVolumeSourceAc {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource {
    fn into_optioned(self) -> RBDPersistentVolumeSourceAc {
        RBDPersistentVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            image: Some(crate::OptionableConvert::into_optioned(self.image)),
            keyring: crate::OptionableConvert::into_optioned(self.keyring),
            monitors: Some(crate::OptionableConvert::into_optioned(self.monitors)),
            pool: crate::OptionableConvert::into_optioned(self.pool),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: RBDPersistentVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            image: crate::OptionableConvert::try_from_optioned(
                value
                    .image
                    .ok_or(crate::optionable::Error {
                        missing_field: "image",
                    })?,
            )?,
            keyring: crate::OptionableConvert::try_from_optioned(value.keyring)?,
            monitors: crate::OptionableConvert::try_from_optioned(
                value
                    .monitors
                    .ok_or(crate::optionable::Error {
                        missing_field: "monitors",
                    })?,
            )?,
            pool: crate::OptionableConvert::try_from_optioned(value.pool)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: RBDPersistentVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.image {
            crate::OptionableConvert::merge(&mut self.image, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.keyring, other.keyring)?;
        if let Some(other_value) = other.monitors {
            crate::OptionableConvert::merge(&mut self.monitors, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.pool, other.pool)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
