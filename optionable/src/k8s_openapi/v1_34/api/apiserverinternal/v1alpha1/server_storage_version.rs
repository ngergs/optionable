pub struct ServerStorageVersionOpt {
    pub api_server_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub decodable_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub encoding_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub served_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::server_storage_version::ServerStorageVersion {
    type Optioned = ServerStorageVersionOpt;
}
#[automatically_derived]
impl crate::Optionable for ServerStorageVersionOpt {
    type Optioned = ServerStorageVersionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::server_storage_version::ServerStorageVersion {
    fn into_optioned(self) -> ServerStorageVersionOpt {
        ServerStorageVersionOpt {
            api_server_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_server_id),
            decodable_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.decodable_versions),
            encoding_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.encoding_version),
            served_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.served_versions),
        }
    }
    fn try_from_optioned(
        value: ServerStorageVersionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_server_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_server_id)?,
            decodable_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.decodable_versions)?,
            encoding_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.encoding_version)?,
            served_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.served_versions)?,
        })
    }
    fn merge(
        &mut self,
        other: ServerStorageVersionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.api_server_id,
            other.api_server_id,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.decodable_versions,
            other.decodable_versions,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.encoding_version,
            other.encoding_version,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.served_versions,
            other.served_versions,
        )?;
        Ok(())
    }
}
