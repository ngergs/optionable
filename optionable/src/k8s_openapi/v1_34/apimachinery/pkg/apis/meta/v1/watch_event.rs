pub enum WatchEventOpt<T>
where
    T: crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized,
{
    Added(Option<<T as crate::Optionable>::Optioned>),
    Deleted(Option<<T as crate::Optionable>::Optioned>),
    Modified(Option<<T as crate::Optionable>::Optioned>),
    Bookmark {
        annotations: Option<
            <std::collections::BTreeMap<
                std::string::String,
                std::string::String,
            > as crate::Optionable>::Optioned,
        >,
        resource_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    },
    ErrorStatus(
        Option<
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as crate::Optionable>::Optioned,
        >,
    ),
    ErrorOther(
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
    <T as crate::Optionable>::Optioned: Sized,
{
    type Optioned = WatchEventOpt<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for WatchEventOpt<T>
where
    T: crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized,
{
    type Optioned = WatchEventOpt<T>;
}
#[automatically_derived]
impl<T> crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::WatchEvent<T>
where
    T: crate::OptionableConvert,
    <T as crate::Optionable>::Optioned: Sized,
{
    fn into_optioned(self) -> WatchEventOpt<T> {
        match self {
            Self::Added(self_0) => {
                WatchEventOpt::Added(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Deleted(self_0) => {
                WatchEventOpt::Deleted(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Modified(self_0) => {
                WatchEventOpt::Modified(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::Bookmark {
                annotations: self_annotations,
                resource_version: self_resource_version,
            } => {
                WatchEventOpt::Bookmark {
                    annotations: Some(
                        crate::OptionableConvert::into_optioned(self_annotations),
                    ),
                    resource_version: Some(
                        crate::OptionableConvert::into_optioned(self_resource_version),
                    ),
                }
            }
            Self::ErrorStatus(self_0) => {
                WatchEventOpt::ErrorStatus(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
            Self::ErrorOther(self_0) => {
                WatchEventOpt::ErrorOther(
                    Some(crate::OptionableConvert::into_optioned(self_0)),
                )
            }
        }
    }
    fn try_from_optioned(
        other: WatchEventOpt<T>,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            match other {
                WatchEventOpt::Added(other_0) => {
                    Self::Added(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                WatchEventOpt::Deleted(other_0) => {
                    Self::Deleted(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                WatchEventOpt::Modified(other_0) => {
                    Self::Modified(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                WatchEventOpt::Bookmark {
                    annotations: other_annotations,
                    resource_version: other_resource_version,
                } => {
                    Self::Bookmark {
                        annotations: crate::OptionableConvert::try_from_optioned(
                            other_annotations
                                .ok_or(crate::optionable::Error {
                                    missing_field: "annotations",
                                })?,
                        )?,
                        resource_version: crate::OptionableConvert::try_from_optioned(
                            other_resource_version
                                .ok_or(crate::optionable::Error {
                                    missing_field: "resource_version",
                                })?,
                        )?,
                    }
                }
                WatchEventOpt::ErrorStatus(other_0) => {
                    Self::ErrorStatus(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
                WatchEventOpt::ErrorOther(other_0) => {
                    Self::ErrorOther(
                        crate::OptionableConvert::try_from_optioned(
                            other_0
                                .ok_or(crate::optionable::Error {
                                    missing_field: "0",
                                })?,
                        )?,
                    )
                }
            },
        )
    }
    fn merge(
        &mut self,
        other: WatchEventOpt<T>,
    ) -> Result<(), crate::optionable::Error> {
        match other {
            WatchEventOpt::Added(other_0) => {
                if let Self::Added(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventOpt::Added(other_0))?;
                }
            }
            WatchEventOpt::Deleted(other_0) => {
                if let Self::Deleted(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventOpt::Deleted(other_0))?;
                }
            }
            WatchEventOpt::Modified(other_0) => {
                if let Self::Modified(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventOpt::Modified(other_0))?;
                }
            }
            WatchEventOpt::Bookmark {
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
                    *self = Self::try_from_optioned(WatchEventOpt::Bookmark {
                        annotations: other_annotations,
                        resource_version: other_resource_version,
                    })?;
                }
            }
            WatchEventOpt::ErrorStatus(other_0) => {
                if let Self::ErrorStatus(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(
                        WatchEventOpt::ErrorStatus(other_0),
                    )?;
                }
            }
            WatchEventOpt::ErrorOther(other_0) => {
                if let Self::ErrorOther(self_0) = self {
                    if let Some(other_value) = other_0 {
                        crate::OptionableConvert::merge(self_0, other_value)?;
                    }
                } else {
                    *self = Self::try_from_optioned(WatchEventOpt::ErrorOther(other_0))?;
                }
            }
        }
        Ok(())
    }
}
