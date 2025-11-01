pub struct ObjectFieldSelectorAc {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub field_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ObjectFieldSelector {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectFieldSelectorAc {
    type Optioned = ObjectFieldSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ObjectFieldSelector {
    fn into_optioned(self) -> ObjectFieldSelectorAc {
        ObjectFieldSelectorAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            field_path: Some(crate::OptionableConvert::into_optioned(self.field_path)),
        }
    }
    fn try_from_optioned(
        value: ObjectFieldSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            field_path: crate::OptionableConvert::try_from_optioned(
                value
                    .field_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "field_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ObjectFieldSelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.field_path {
            crate::OptionableConvert::merge(&mut self.field_path, other_value)?;
        }
        Ok(())
    }
}
