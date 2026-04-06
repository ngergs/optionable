#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RBDPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyring: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::RBDPersistentVolumeSource {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for RBDPersistentVolumeSourceAc {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::RBDPersistentVolumeSource {
    fn into_optioned(self) -> RBDPersistentVolumeSourceAc {
        RBDPersistentVolumeSourceAc {
            fs_type: self.fs_type,
            image: Some(self.image),
            keyring: self.keyring,
            monitors: Some(self.monitors),
            pool: self.pool,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: self.user,
        }
    }
    fn try_from_optioned(
        value: RBDPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            image: value
                .image
                .ok_or(crate::Error {
                    missing_field: "image",
                })?,
            keyring: value.keyring,
            monitors: value
                .monitors
                .ok_or(crate::Error {
                    missing_field: "monitors",
                })?,
            pool: value.pool,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: value.user,
        })
    }
    fn merge(&mut self, other: RBDPersistentVolumeSourceAc) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        if let Some(other_value) = other.image {
            self.image = other_value;
        }
        self.keyring = other.keyring;
        if let Some(other_value) = other.monitors {
            self.monitors = other_value;
        }
        self.pool = other.pool;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        self.user = other.user;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::RBDPersistentVolumeSource>
for RBDPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::RBDPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::RBDPersistentVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::RBDPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
