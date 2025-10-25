pub struct FileKeySelectorOpt {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub volume_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::file_key_selector::FileKeySelector {
    type Optioned = FileKeySelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for FileKeySelectorOpt {
    type Optioned = FileKeySelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::file_key_selector::FileKeySelector {
    fn into_optioned(self) -> FileKeySelectorOpt {
        FileKeySelectorOpt {
            key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.key,
                ),
            ),
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.optional),
            path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.path,
                ),
            ),
            volume_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.volume_name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: FileKeySelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.optional)?,
            path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            volume_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: FileKeySelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.key,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.optional, other.optional)?;
        if let Some(other_value) = other.path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.path,
                other_value,
            )?;
        }
        if let Some(other_value) = other.volume_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.volume_name,
                other_value,
            )?;
        }
        Ok(())
    }
}
