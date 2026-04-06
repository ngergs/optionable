#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FCVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "targetWWNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wwns: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwids: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FCVolumeSource {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FCVolumeSourceAc {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::FCVolumeSource {
    fn into_optioned(self) -> FCVolumeSourceAc {
        FCVolumeSourceAc {
            fs_type: self.fs_type,
            lun: self.lun,
            read_only: self.read_only,
            target_wwns: self.target_wwns,
            wwids: self.wwids,
        }
    }
    fn try_from_optioned(value: FCVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            lun: value.lun,
            read_only: value.read_only,
            target_wwns: value.target_wwns,
            wwids: value.wwids,
        })
    }
    fn merge(&mut self, other: FCVolumeSourceAc) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        self.lun = other.lun;
        self.read_only = other.read_only;
        self.target_wwns = other.target_wwns;
        self.wwids = other.wwids;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FCVolumeSource>
for FCVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::FCVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::FCVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FCVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
