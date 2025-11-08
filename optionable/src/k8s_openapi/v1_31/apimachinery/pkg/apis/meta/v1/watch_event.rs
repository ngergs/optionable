#[derive(Clone, std::fmt::Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum WatchEventAc<T>
where
    T: crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + PartialEq
        + serde::Serialize + serde::de::DeserializeOwned,
{
    Added(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<T as crate::Optionable>::Optioned>,
    ),
    Deleted(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<T as crate::Optionable>::Optioned>,
    ),
    Modified(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<<T as crate::Optionable>::Optioned>,
    ),
    Bookmark {
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<
            <std::collections::BTreeMap<
                std::string::String,
                std::string::String,
            > as crate::Optionable>::Optioned,
        >,
        #[serde(skip_serializing_if = "Option::is_none")]
        resource_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    },
    ErrorStatus(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as crate::Optionable>::Optioned,
        >,
    ),
    ErrorOther(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<
            <::k8s_openapi::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
        >,
    ),
}
#[automatically_derived]
impl<T> crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>
where
    T: crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + PartialEq
        + serde::Serialize + serde::de::DeserializeOwned,
{
    type Optioned = WatchEventAc<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for WatchEventAc<T>
where
    T: crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + PartialEq
        + serde::Serialize + serde::de::DeserializeOwned,
{
    type Optioned = WatchEventAc<T>;
}
#[automatically_derived]
impl<T> crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>
where
    T: crate::OptionableConvert,
    <T as crate::Optionable>::Optioned: Sized + Clone + std::fmt::Debug + PartialEq
        + serde::Serialize + serde::de::DeserializeOwned,
{
    fn into_optioned(self) -> WatchEventAc<T> {
        match self {
            Self::Added(self_0) => {
                WatchEventAc::Added(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Deleted(self_0) => {
                WatchEventAc::Deleted(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Modified(self_0) => {
                WatchEventAc::Modified(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Bookmark {
                annotations: self_annotations,
                resource_version: self_resource_version,
            } => {
                WatchEventAc::Bookmark {
                    annotations: Some(
                        crate::OptionableConvert::into_optioned(self_annotations),
                    ),
                    resource_version: Some(
                        crate::OptionableConvert::into_optioned(self_resource_version),
                    ),
                }
            }
            Self::ErrorStatus(self_0) => {
                WatchEventAc::ErrorStatus(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::ErrorOther(self_0) => {
                WatchEventAc::ErrorOther(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(other: WatchEventAc<T>) -> Result<Self, crate::Error> {
        Ok(
            match other {
                WatchEventAc::Added(other_0) => {
                    Self::Added(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                WatchEventAc::Deleted(other_0) => {
                    Self::Deleted(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                WatchEventAc::Modified(other_0) => {
                    Self::Modified(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                WatchEventAc::Bookmark {
                    annotations: other_annotations,
                    resource_version: other_resource_version,
                } => {
                    Self::Bookmark {
                        annotations: crate::OptionableConvert::try_from_optioned(
                            other_annotations
                                .ok_or(crate::Error {
                                    missing_field: "annotations",
                                })?,
                        )?,
                        resource_version: crate::OptionableConvert::try_from_optioned(
                            other_resource_version
                                .ok_or(crate::Error {
                                    missing_field: "resource_version",
                                })?,
                        )?,
                    }
                }
                WatchEventAc::ErrorStatus(other_0) => {
                    Self::ErrorStatus(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
                WatchEventAc::ErrorOther(other_0) => {
                    Self::ErrorOther(
                        crate::OptionableConvert::try_from_optioned(
                            other_0.ok_or(crate::Error { missing_field: "0" })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(&mut self, other: WatchEventAc<T>) -> Result<(), crate::Error> {
        match other {
            WatchEventAc::Added(other_0) => {
                if let Self::Added(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::Added(other_0))?;
                }
            }
            WatchEventAc::Deleted(other_0) => {
                if let Self::Deleted(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::Deleted(other_0))?;
                }
            }
            WatchEventAc::Modified(other_0) => {
                if let Self::Modified(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::Modified(other_0))?;
                }
            }
            WatchEventAc::Bookmark {
                annotations: other_annotations,
                resource_version: other_resource_version,
            } => {
                if let Self::Bookmark {
                    annotations: self_annotations,
                    resource_version: self_resource_version,
                } = self {
                    if let Some(other_value) = other_annotations {
                        crate::OptionableConvert::merge(self_annotations, other_value)?;
                    }
                    if let Some(other_value) = other_resource_version {
                        crate::OptionableConvert::merge(
                            self_resource_version,
                            other_value,
                        )?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::Bookmark {
                        annotations: other_annotations,
                        resource_version: other_resource_version,
                    })?;
                }
            }
            WatchEventAc::ErrorStatus(other_0) => {
                if let Self::ErrorStatus(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::ErrorStatus(other_0))?;
                }
            }
            WatchEventAc::ErrorOther(other_0) => {
                if let Self::ErrorOther(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventAc::ErrorOther(other_0))?;
                }
            }
        }
        Ok(())
    }
}
