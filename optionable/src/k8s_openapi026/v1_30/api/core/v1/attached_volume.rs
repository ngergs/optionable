#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttachedVolumeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::AttachedVolume {
    type Optioned = AttachedVolumeAc;
}
#[automatically_derived]
impl crate::Optionable for AttachedVolumeAc {
    type Optioned = AttachedVolumeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::AttachedVolume {
    fn into_optioned(self) -> AttachedVolumeAc {
        AttachedVolumeAc {
            device_path: Some(crate::OptionableConvert::into_optioned(self.device_path)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: AttachedVolumeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device_path: crate::OptionableConvert::try_from_optioned(
                value
                    .device_path
                    .ok_or(crate::Error {
                        missing_field: "device_path",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: AttachedVolumeAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.device_path {
            crate::OptionableConvert::merge(&mut self.device_path, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::AttachedVolume>
for AttachedVolumeAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::AttachedVolume) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::AttachedVolume, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::AttachedVolume,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
