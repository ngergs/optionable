#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceColumnDefinitionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition {
    type Optioned = CustomResourceColumnDefinitionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceColumnDefinitionAc {
    type Optioned = CustomResourceColumnDefinitionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition {
    fn into_optioned(self) -> CustomResourceColumnDefinitionAc {
        CustomResourceColumnDefinitionAc {
            description: crate::OptionableConvert::into_optioned(self.description),
            format: crate::OptionableConvert::into_optioned(self.format),
            json_path: Some(crate::OptionableConvert::into_optioned(self.json_path)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            priority: crate::OptionableConvert::into_optioned(self.priority),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: CustomResourceColumnDefinitionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            description: crate::OptionableConvert::try_from_optioned(value.description)?,
            format: crate::OptionableConvert::try_from_optioned(value.format)?,
            json_path: crate::OptionableConvert::try_from_optioned(
                value
                    .json_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "json_path",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            priority: crate::OptionableConvert::try_from_optioned(value.priority)?,
            type_: crate::OptionableConvert::try_from_optioned(
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
        other: CustomResourceColumnDefinitionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.description, other.description)?;
        crate::OptionableConvert::merge(&mut self.format, other.format)?;
        if let Some(other_value) = other.json_path {
            crate::OptionableConvert::merge(&mut self.json_path, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.priority, other.priority)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
