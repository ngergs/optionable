pub struct APIResourceAc {
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
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource {
    type Optioned = APIResourceAc;
}
#[automatically_derived]
impl crate::Optionable for APIResourceAc {
    type Optioned = APIResourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource {
    fn into_optioned(self) -> APIResourceAc {
        APIResourceAc {
            categories: crate::OptionableConvert::into_optioned(self.categories),
            group: crate::OptionableConvert::into_optioned(self.group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespaced: Some(self.namespaced),
            short_names: crate::OptionableConvert::into_optioned(self.short_names),
            singular_name: Some(
                crate::OptionableConvert::into_optioned(self.singular_name),
            ),
            storage_version_hash: crate::OptionableConvert::into_optioned(
                self.storage_version_hash,
            ),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: APIResourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            categories: crate::OptionableConvert::try_from_optioned(value.categories)?,
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
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
            short_names: crate::OptionableConvert::try_from_optioned(value.short_names)?,
            singular_name: crate::OptionableConvert::try_from_optioned(
                value
                    .singular_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "singular_name",
                    })?,
            )?,
            storage_version_hash: crate::OptionableConvert::try_from_optioned(
                value.storage_version_hash,
            )?,
            verbs: crate::OptionableConvert::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(&mut self, other: APIResourceAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.categories, other.categories)?;
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.namespaced {
            self.namespaced = other_value;
        }
        crate::OptionableConvert::merge(&mut self.short_names, other.short_names)?;
        if let Some(other_value) = other.singular_name {
            crate::OptionableConvert::merge(&mut self.singular_name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.storage_version_hash,
            other.storage_version_hash,
        )?;
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
