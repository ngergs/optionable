#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct LeaseAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1::LeaseSpec,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<LeaseAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1::Lease {
    type Optioned = LeaseAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseAc {
    type Optioned = LeaseAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::coordination::v1::Lease {
    fn into_optioned(self) -> LeaseAc {
        LeaseAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: LeaseAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: LeaseAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for LeaseAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for LeaseAc {
    type Ty = <::k8s_openapi::api::coordination::v1::Lease as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
