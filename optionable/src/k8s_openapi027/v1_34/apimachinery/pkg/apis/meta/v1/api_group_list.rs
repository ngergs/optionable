#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIGroupListAc {
    #[serde(
        serialize_with = "crate::k8s_openapi027::serialize_api_version",
        deserialize_with = "crate::k8s_openapi027::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi027::serialize_kind",
        deserialize_with = "crate::k8s_openapi027::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<
        <std::vec::Vec<
            ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupListAc {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList {
    fn into_optioned(self) -> APIGroupListAc {
        APIGroupListAc {
            api_version: Default::default(),
            kind: Default::default(),
            groups: Some(crate::OptionableConvert::into_optioned(self.groups)),
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList,
> for APIGroupListAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for APIGroupListAc {
    const API_VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList as k8s_openapi027::Resource>::Scope;
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apigrouplistac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroupList,
    >();
}
