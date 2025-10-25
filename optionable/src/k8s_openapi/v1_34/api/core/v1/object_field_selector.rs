pub struct ObjectFieldSelectorOpt {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub field_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::object_field_selector::ObjectFieldSelector {
    type Optioned = ObjectFieldSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for ObjectFieldSelectorOpt {
    type Optioned = ObjectFieldSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::object_field_selector::ObjectFieldSelector {
    fn into_optioned(self) -> ObjectFieldSelectorOpt {
        ObjectFieldSelectorOpt {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_version),
            field_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.field_path,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ObjectFieldSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_version)?,
            field_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: ObjectFieldSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.field_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.field_path,
                other_value,
            )?;
        }
        Ok(())
    }
}
