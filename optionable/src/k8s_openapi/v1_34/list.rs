#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<<std::vec::Vec<T> as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ListAc<T>>,
}
#[automatically_derived]
impl<T> crate::Optionable for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::OptionableConvert for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource + crate::OptionableConvert,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    fn into_optioned(self) -> ListAc<T> {
        ListAc::<T> {
            items: Some(crate::OptionableConvert::into_optioned(self.items)),
            metadata: self.metadata,
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: ListAc<T>) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(
                value
                    .items
                    .ok_or(crate::optionable::Error {
                        missing_field: "items",
                    })?,
            )?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ListAc<T>) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.items {
            crate::OptionableConvert::merge(&mut self.items, other_value)?;
        }
        self.metadata = other.metadata;
        Ok(())
    }
}
impl<T> k8s_openapi::Resource for ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    const API_VERSION: &'static str = <::k8s_openapi::List<
        T,
    > as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::List<T> as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::List<T> as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::List<
        T,
    > as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::List<
        T,
    > as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::List<T> as k8s_openapi::Resource>::Scope;
}
impl<T> k8s_openapi::Metadata for ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + Default
        + PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    type Ty = <::k8s_openapi::List<T> as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
