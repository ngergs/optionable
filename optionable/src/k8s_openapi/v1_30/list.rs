#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListAc<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::Optionable,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<<std::vec::Vec<T> as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta,
}
#[automatically_derived]
impl<T> crate::Optionable for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::Optionable,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for ListAc<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::Optionable,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl<T> crate::OptionableConvert for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::OptionableConvert,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    fn into_optioned(self) -> ListAc<T> {
        ListAc::<T> {
            api_version: Default::default(),
            kind: Default::default(),
            items: Some(crate::OptionableConvert::into_optioned(self.items)),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ListAc<T>) -> Result<Self, crate::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(
                value
                    .items
                    .ok_or(crate::Error {
                        missing_field: "items",
                    })?,
            )?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ListAc<T>) -> Result<(), crate::Error> {
        if let Some(other_value) = other.items {
            crate::OptionableConvert::merge(&mut self.items, other_value)?;
        }
        self.metadata = other.metadata;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl<T> crate::OptionedConvert<::k8s_openapi::List<T>> for ListAc<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::OptionableConvert,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    fn from_optionable(value: ::k8s_openapi::List<T>) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(self) -> Result<::k8s_openapi::List<T>, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(self, other: &mut ::k8s_openapi::List<T>) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl<T> k8s_openapi::Resource for ListAc<T>
where
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::Optionable,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
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
    T: k8s_openapi::ListableResource,
    std::vec::Vec<T>: crate::Optionable,
    <std::vec::Vec<
        T,
    > as crate::Optionable>::Optioned: Sized + Clone + Default + PartialEq
        + serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    type Ty = <::k8s_openapi::List<T> as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
