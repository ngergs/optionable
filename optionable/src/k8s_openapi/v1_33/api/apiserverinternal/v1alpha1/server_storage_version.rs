#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ServerStorageVersionAc {
    #[serde(rename = "apiServerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_server_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decodable_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion {
    type Optioned = ServerStorageVersionAc;
}
#[automatically_derived]
impl crate::Optionable for ServerStorageVersionAc {
    type Optioned = ServerStorageVersionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion {
    fn into_optioned(self) -> ServerStorageVersionAc {
        ServerStorageVersionAc {
            api_server_id: crate::OptionableConvert::into_optioned(self.api_server_id),
            decodable_versions: crate::OptionableConvert::into_optioned(
                self.decodable_versions,
            ),
            encoding_version: crate::OptionableConvert::into_optioned(
                self.encoding_version,
            ),
            served_versions: crate::OptionableConvert::into_optioned(
                self.served_versions,
            ),
        }
    }
    fn try_from_optioned(value: ServerStorageVersionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_server_id: crate::OptionableConvert::try_from_optioned(
                value.api_server_id,
            )?,
            decodable_versions: crate::OptionableConvert::try_from_optioned(
                value.decodable_versions,
            )?,
            encoding_version: crate::OptionableConvert::try_from_optioned(
                value.encoding_version,
            )?,
            served_versions: crate::OptionableConvert::try_from_optioned(
                value.served_versions,
            )?,
        })
    }
    fn merge(&mut self, other: ServerStorageVersionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_server_id, other.api_server_id)?;
        crate::OptionableConvert::merge(
            &mut self.decodable_versions,
            other.decodable_versions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.encoding_version,
            other.encoding_version,
        )?;
        crate::OptionableConvert::merge(
            &mut self.served_versions,
            other.served_versions,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
> for ServerStorageVersionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
