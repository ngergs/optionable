#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct APIGroupListAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<APIGroupListAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupListAc {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList {
    fn into_optioned(self) -> APIGroupListAc {
        APIGroupListAc {
            groups: Some(crate::OptionableConvert::into_optioned(self.groups)),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: APIGroupListAc) -> Result<Self, crate::Error> {
        Ok(Self {
            groups: crate::OptionableConvert::try_from_optioned(
                value
                    .groups
                    .ok_or(crate::Error {
                        missing_field: "groups",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIGroupListAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.groups {
            crate::OptionableConvert::merge(&mut self.groups, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for APIGroupListAc {
    const API_VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi::Resource>::Scope;
}
