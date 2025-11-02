#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct APIResourceListAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList {
    type Optioned = APIResourceListAc;
}
#[automatically_derived]
impl crate::Optionable for APIResourceListAc {
    type Optioned = APIResourceListAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList {
    fn into_optioned(self) -> APIResourceListAc {
        APIResourceListAc {
            group_version: Some(
                crate::OptionableConvert::into_optioned(self.group_version),
            ),
            resources: Some(crate::OptionableConvert::into_optioned(self.resources)),
        }
    }
    fn try_from_optioned(
        value: APIResourceListAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group_version: crate::OptionableConvert::try_from_optioned(
                value
                    .group_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "group_version",
                    })?,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(
                value
                    .resources
                    .ok_or(crate::optionable::Error {
                        missing_field: "resources",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: APIResourceListAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.group_version {
            crate::OptionableConvert::merge(&mut self.group_version, other_value)?;
        }
        if let Some(other_value) = other.resources {
            crate::OptionableConvert::merge(&mut self.resources, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for APIResourceListAc {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "APIResourceList";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = k8s_openapi::ClusterResourceScope;
}
