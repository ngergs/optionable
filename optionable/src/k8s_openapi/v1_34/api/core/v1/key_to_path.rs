pub struct KeyToPathOpt {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub mode: <Option<i32> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::KeyToPath {
    type Optioned = KeyToPathOpt;
}
#[automatically_derived]
impl crate::Optionable for KeyToPathOpt {
    type Optioned = KeyToPathOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::KeyToPath {
    fn into_optioned(self) -> KeyToPathOpt {
        KeyToPathOpt {
            key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.key,
                ),
            ),
            mode: <Option<i32> as crate::OptionableConvert>::into_optioned(self.mode),
            path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.path,
                ),
            ),
        }
    }
    fn try_from_optioned(value: KeyToPathOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.mode)?,
            path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: KeyToPathOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.key,
                other_value,
            )?;
        }
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.mode, other.mode)?;
        if let Some(other_value) = other.path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.path,
                other_value,
            )?;
        }
        Ok(())
    }
}
