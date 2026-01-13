#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyAc {
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
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi026::api::networking::v1::NetworkPolicySpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::networking::v1::NetworkPolicy {
    type Optioned = NetworkPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyAc {
    type Optioned = NetworkPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::networking::v1::NetworkPolicy {
    fn into_optioned(self) -> NetworkPolicyAc {
        NetworkPolicyAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: NetworkPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: NetworkPolicyAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::networking::v1::NetworkPolicy>
for NetworkPolicyAc {
    fn from_optionable(
        value: k8s_openapi026::api::networking::v1::NetworkPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::networking::v1::NetworkPolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::networking::v1::NetworkPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for NetworkPolicyAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for NetworkPolicyAc {
    type Ty = <k8s_openapi026::api::networking::v1::NetworkPolicy as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_networkpolicyac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::api::networking::v1::NetworkPolicy,
    >();
}
