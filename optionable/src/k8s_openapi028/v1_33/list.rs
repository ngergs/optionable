#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// List is a list of resources.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListAc<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
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
    /// List of objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<std::vec::Vec<<T as crate::Optionable>::Optioned>>,
    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub metadata: ::k8s_openapi028::apimachinery::pkg::apis::meta::v1::ListMeta,
}
#[automatically_derived]
impl<T> crate::Optionable for k8s_openapi028::List<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for ListAc<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Optioned = ListAc<T>;
}
impl<T> k8s_openapi028::Resource for ListAc<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    const API_VERSION: &'static str = <k8s_openapi028::List<
        T,
    > as k8s_openapi028::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi028::List<
        T,
    > as k8s_openapi028::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi028::List<
        T,
    > as k8s_openapi028::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi028::List<
        T,
    > as k8s_openapi028::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi028::List<
        T,
    > as k8s_openapi028::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi028::List<T> as k8s_openapi028::Resource>::Scope;
}
impl<T> k8s_openapi028::Metadata for ListAc<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Ty = <k8s_openapi028::List<T> as k8s_openapi028::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi028::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi028::Metadata>::Ty {
        &mut self.metadata
    }
}
