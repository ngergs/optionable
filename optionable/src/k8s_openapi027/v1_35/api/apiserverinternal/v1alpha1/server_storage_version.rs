#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// An API server instance reports the version it can decode and the version it encodes objects to when persisting objects in the backend.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServerStorageVersionAc {
    /// The ID of the reporting API server.
    #[serde(rename = "apiServerID")]
    pub api_server_id: Option<std::string::String>,
    /// The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decodable_versions: Option<std::vec::Vec<std::string::String>>,
    /// The API server encodes the object to this version when persisting it in the backend (e.g., etcd).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_version: Option<std::string::String>,
    /// The API server can serve these versions. DecodableVersions must include all ServedVersions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_versions: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion {
    type Optioned = ServerStorageVersionAc;
}
#[automatically_derived]
impl crate::Optionable for ServerStorageVersionAc {
    type Optioned = ServerStorageVersionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion {
    fn into_optioned(self) -> ServerStorageVersionAc {
        ServerStorageVersionAc {
            api_server_id: self.api_server_id,
            decodable_versions: self.decodable_versions,
            encoding_version: self.encoding_version,
            served_versions: self.served_versions,
        }
    }
    fn try_from_optioned(value: ServerStorageVersionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_server_id: value.api_server_id,
            decodable_versions: value.decodable_versions,
            encoding_version: value.encoding_version,
            served_versions: value.served_versions,
        })
    }
    fn merge(&mut self, other: ServerStorageVersionAc) -> Result<(), crate::Error> {
        self.api_server_id = other.api_server_id;
        if self.decodable_versions.is_none() {
            self.decodable_versions = crate::OptionableConvert::try_from_optioned(
                other.decodable_versions,
            )?;
        } else if let Some(self_value) = self.decodable_versions.as_mut()
            && let Some(other_value) = other.decodable_versions
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        if self.encoding_version.is_none() {
            self.encoding_version = crate::OptionableConvert::try_from_optioned(
                other.encoding_version,
            )?;
        } else if let Some(self_value) = self.encoding_version.as_mut()
            && let Some(other_value) = other.encoding_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.served_versions.is_none() {
            self.served_versions = crate::OptionableConvert::try_from_optioned(
                other.served_versions,
            )?;
        } else if let Some(self_value) = self.served_versions.as_mut()
            && let Some(other_value) = other.served_versions
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.api_server_id == other.api_server_id
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion,
> for ServerStorageVersionAc {
    fn from_optionable(
        value: k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
