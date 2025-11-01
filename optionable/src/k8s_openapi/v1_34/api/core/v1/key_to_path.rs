pub struct KeyToPathAc {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub mode: <Option<i32> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::KeyToPath {
    type Optioned = KeyToPathAc;
}
#[automatically_derived]
impl crate::Optionable for KeyToPathAc {
    type Optioned = KeyToPathAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::KeyToPath {
    fn into_optioned(self) -> KeyToPathAc {
        KeyToPathAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            mode: crate::OptionableConvert::into_optioned(self.mode),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
        }
    }
    fn try_from_optioned(value: KeyToPathAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            mode: crate::OptionableConvert::try_from_optioned(value.mode)?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: KeyToPathAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.mode, other.mode)?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        Ok(())
    }
}
