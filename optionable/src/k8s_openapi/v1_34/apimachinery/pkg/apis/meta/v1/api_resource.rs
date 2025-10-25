pub struct APIResourceOpt {
    pub categories: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespaced: Option<bool>,
    pub short_names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub singular_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub storage_version_hash: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::api_resource::APIResource {
    type Optioned = APIResourceOpt;
}
#[automatically_derived]
impl crate::Optionable for APIResourceOpt {
    type Optioned = APIResourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::api_resource::APIResource {
    fn into_optioned(self) -> APIResourceOpt {
        APIResourceOpt {
            categories: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.categories),
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            kind: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kind,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            namespaced: Some(self.namespaced),
            short_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.short_names),
            singular_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.singular_name,
                ),
            ),
            storage_version_hash: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.storage_version_hash),
            verbs: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.verbs),
            ),
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: APIResourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            categories: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.categories)?,
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            kind: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespaced: value
                .namespaced
                .ok_or(crate::optionable::Error {
                    missing_field: "namespaced",
                })?,
            short_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.short_names)?,
            singular_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .singular_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "singular_name",
                    })?,
            )?,
            storage_version_hash: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.storage_version_hash,
            )?,
            verbs: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.version)?,
        })
    }
    fn merge(&mut self, other: APIResourceOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.categories, other.categories)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.kind {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kind,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.namespaced {
            self.namespaced = other_value;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.short_names, other.short_names)?;
        if let Some(other_value) = other.singular_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.singular_name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_version_hash,
            other.storage_version_hash,
        )?;
        if let Some(other_value) = other.verbs {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.verbs, other_value)?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
