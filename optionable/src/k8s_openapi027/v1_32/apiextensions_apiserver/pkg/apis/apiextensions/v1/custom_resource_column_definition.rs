#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceColumnDefinitionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition {
    type Optioned = CustomResourceColumnDefinitionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceColumnDefinitionAc {
    type Optioned = CustomResourceColumnDefinitionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition {
    fn into_optioned(self) -> CustomResourceColumnDefinitionAc {
        CustomResourceColumnDefinitionAc {
            description: self.description,
            format: self.format,
            json_path: Some(self.json_path),
            name: Some(self.name),
            priority: self.priority,
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(
        value: CustomResourceColumnDefinitionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            description: value.description,
            format: value.format,
            json_path: value
                .json_path
                .ok_or(crate::Error {
                    missing_field: "json_path",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            priority: value.priority,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceColumnDefinitionAc,
    ) -> Result<(), crate::Error> {
        self.description = other.description;
        self.format = other.format;
        if let Some(other_value) = other.json_path {
            self.json_path = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.priority = other.priority;
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
> for CustomResourceColumnDefinitionAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
