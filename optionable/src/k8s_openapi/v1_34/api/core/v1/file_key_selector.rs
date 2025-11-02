#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileKeySelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FileKeySelector {
    type Optioned = FileKeySelectorAc;
}
#[automatically_derived]
impl crate::Optionable for FileKeySelectorAc {
    type Optioned = FileKeySelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::FileKeySelector {
    fn into_optioned(self) -> FileKeySelectorAc {
        FileKeySelectorAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            optional: crate::OptionableConvert::into_optioned(self.optional),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            volume_name: Some(crate::OptionableConvert::into_optioned(self.volume_name)),
        }
    }
    fn try_from_optioned(
        value: FileKeySelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            optional: crate::OptionableConvert::try_from_optioned(value.optional)?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            volume_name: crate::OptionableConvert::try_from_optioned(
                value
                    .volume_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "volume_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: FileKeySelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.optional, other.optional)?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        if let Some(other_value) = other.volume_name {
            crate::OptionableConvert::merge(&mut self.volume_name, other_value)?;
        }
        Ok(())
    }
}
