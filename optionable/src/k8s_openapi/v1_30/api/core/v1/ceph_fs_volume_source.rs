#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct CephFSVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_file: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::CephFSVolumeSource {
    type Optioned = CephFSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CephFSVolumeSourceAc {
    type Optioned = CephFSVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::CephFSVolumeSource {
    fn into_optioned(self) -> CephFSVolumeSourceAc {
        CephFSVolumeSourceAc {
            monitors: Some(crate::OptionableConvert::into_optioned(self.monitors)),
            path: crate::OptionableConvert::into_optioned(self.path),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_file: crate::OptionableConvert::into_optioned(self.secret_file),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: CephFSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            monitors: crate::OptionableConvert::try_from_optioned(
                value
                    .monitors
                    .ok_or(crate::Error {
                        missing_field: "monitors",
                    })?,
            )?,
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_file: crate::OptionableConvert::try_from_optioned(value.secret_file)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: CephFSVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.monitors {
            crate::OptionableConvert::merge(&mut self.monitors, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_file, other.secret_file)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
