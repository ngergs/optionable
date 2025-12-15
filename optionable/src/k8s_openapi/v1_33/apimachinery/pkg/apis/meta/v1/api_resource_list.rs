#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIResourceListAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList {
    fn into_optioned(self) -> APIResourceListAc {
        APIResourceListAc {
            api_version: Default::default(),
            kind: Default::default(),
            group_version: Some(
                crate::OptionableConvert::into_optioned(self.group_version),
            ),
            resources: Some(crate::OptionableConvert::into_optioned(self.resources)),
        }
    }
    fn try_from_optioned(value: APIResourceListAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group_version: crate::OptionableConvert::try_from_optioned(
                value
                    .group_version
                    .ok_or(crate::Error {
                        missing_field: "group_version",
                    })?,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(
                value
                    .resources
                    .ok_or(crate::Error {
                        missing_field: "resources",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIResourceListAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.group_version {
            crate::OptionableConvert::merge(&mut self.group_version, other_value)?;
        }
        if let Some(other_value) = other.resources {
            crate::OptionableConvert::merge(&mut self.resources, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList,
> for APIResourceListAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for APIResourceListAc {
    const API_VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList as k8s_openapi::Resource>::Scope;
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apiresourcelistac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList,
    >();
}
