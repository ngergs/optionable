use crate::k8s_openapi::ListAc;
use crate::optionable::impl_optional_self;
use k8s_openapi028::ByteString;

impl_optional_self!(ByteString);

#[cfg(feature = "k8s_openapi_convert")]
impl<T> crate::OptionableConvert for k8s_openapi028::List<T>
where
    T: k8s_openapi028::ListableResource
        + crate::OptionableConvert
        + k8s_openapi028::Metadata<Ty = k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    <T as crate::Optionable>::Optioned: Sized
        + Clone
        + Default
        + PartialEq
        + serde::de::DeserializeOwned
        + serde::Serialize
        + std::fmt::Debug
        + k8s_openapi028::Metadata<Ty = k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
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
            items: crate::OptionableConvert::try_from_optioned(value.items.ok_or(
                crate::Error {
                    missing_field: "items",
                },
            )?)?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ListAc<T>) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(items_other) = other.items {
            for el_other in items_other {
                if let Some(el_self) = self.items.iter_mut().find(|el_self| {
                    let meta_self = k8s_openapi028::Metadata::metadata(*el_self);
                    let meta_other = k8s_openapi028::Metadata::metadata(&el_other);
                    meta_self.name == meta_other.name && meta_self.namespace == meta_other.namespace
                }) {
                    crate::OptionableConvert::merge(el_self, el_other)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "k8s_openapi_convert")]
impl<T> crate::OptionedConvert<k8s_openapi028::List<T>> for ListAc<T>
where
    T: k8s_openapi028::ListableResource
        + crate::OptionableConvert
        + k8s_openapi028::Metadata<Ty = k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    <T as crate::Optionable>::Optioned: Sized
        + Clone
        + Default
        + PartialEq
        + serde::de::DeserializeOwned
        + serde::Serialize
        + std::fmt::Debug
        + k8s_openapi028::Metadata<Ty = k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
{
    fn from_optionable(value: k8s_openapi028::List<T>) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(self) -> Result<k8s_openapi028::List<T>, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(self, other: &mut k8s_openapi028::List<T>) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}

impl<T> k8s_openapi028::DeepMerge for ListAc<T>
where
    T: k8s_openapi028::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized
        + Clone
        + Default
        + PartialEq
        + serde::de::DeserializeOwned
        + serde::Serialize
        + std::fmt::Debug
        + k8s_openapi028::Metadata<Ty = k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta>
        + k8s_openapi028::DeepMerge,
{
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        if let Some(items_other) = other.items {
            if let Some(items_self) = &mut self.items {
                for el_other in items_other {
                    if let Some(el_self) = items_self.into_iter().find(|el_self| {
                        let meta_self = k8s_openapi028::Metadata::metadata(*el_self);
                        let meta_other = k8s_openapi028::Metadata::metadata(&el_other);
                        meta_self.name == meta_other.name
                            && meta_self.namespace == meta_other.namespace
                    }) {
                        k8s_openapi028::DeepMerge::merge_from(el_self, el_other);
                    }
                }
            } else {
                self.items = Some(items_other);
            }
        }
    }
}
