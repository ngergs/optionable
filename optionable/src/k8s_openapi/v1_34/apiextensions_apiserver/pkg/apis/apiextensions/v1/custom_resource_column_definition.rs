pub struct CustomResourceColumnDefinitionOpt {
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub format: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub json_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub priority: <Option<i32> as crate::Optionable>::Optioned,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_column_definition::CustomResourceColumnDefinition {
    type Optioned = CustomResourceColumnDefinitionOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceColumnDefinitionOpt {
    type Optioned = CustomResourceColumnDefinitionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_column_definition::CustomResourceColumnDefinition {
    fn into_optioned(self) -> CustomResourceColumnDefinitionOpt {
        CustomResourceColumnDefinitionOpt {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.description),
            format: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.format),
            json_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.json_path,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            priority: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.priority),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceColumnDefinitionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.description)?,
            format: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.format)?,
            json_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .json_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "json_path",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            priority: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.priority)?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceColumnDefinitionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.description, other.description)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.format, other.format)?;
        if let Some(other_value) = other.json_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.json_path,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.priority, other.priority)?;
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
