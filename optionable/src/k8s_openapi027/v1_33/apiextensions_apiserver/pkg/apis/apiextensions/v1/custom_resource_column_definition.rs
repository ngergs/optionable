#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceColumnDefinition specifies a column for server side printing.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceColumnDefinitionAc {
    /// description is a human readable description of this column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<std::string::String>,
    /// format is an optional OpenAPI type definition for this column. The 'name' format is applied to the primary identifier column to assist in clients identifying column is the resource name. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<std::string::String>,
    /// jsonPath is a simple JSON path (i.e. with array notation) which is evaluated against each custom resource to produce the value for this column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<std::string::String>,
    /// name is a human readable name for the column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// priority is an integer defining the relative importance of this column compared to others. Lower numbers are considered higher priority. Columns that may be omitted in limited space scenarios should be given a priority greater than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// type is an OpenAPI type definition for this column. See https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types for details.
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
        if self.description.is_none() {
            self.description = crate::OptionableConvert::try_from_optioned(
                other.description,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.description, other.description)?;
        }
        if self.format.is_none() {
            self.format = crate::OptionableConvert::try_from_optioned(other.format)?;
        } else {
            crate::OptionableConvert::merge(&mut self.format, other.format)?;
        }
        if let Some(other_value) = other.json_path {
            self.json_path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.priority.is_none() {
            self.priority = crate::OptionableConvert::try_from_optioned(other.priority)?;
        } else {
            crate::OptionableConvert::merge(&mut self.priority, other.priority)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
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
