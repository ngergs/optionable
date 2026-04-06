#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JSONSchemaPropsAc {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_path: Option<std::string::String>,
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<std::string::String>,
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<
        std::boxed::Box<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_embedded_resource: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_int_or_string: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_list_map_keys: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_list_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_map_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_validations: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps {
    type Optioned = JSONSchemaPropsAc;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsAc {
    type Optioned = JSONSchemaPropsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps {
    fn into_optioned(self) -> JSONSchemaPropsAc {
        JSONSchemaPropsAc {
            ref_path: self.ref_path,
            schema: self.schema,
            additional_items: crate::OptionableConvert::into_optioned(
                self.additional_items,
            ),
            additional_properties: crate::OptionableConvert::into_optioned(
                self.additional_properties,
            ),
            all_of: crate::OptionableConvert::into_optioned(self.all_of),
            any_of: crate::OptionableConvert::into_optioned(self.any_of),
            default: crate::OptionableConvert::into_optioned(self.default),
            definitions: crate::OptionableConvert::into_optioned(self.definitions),
            dependencies: crate::OptionableConvert::into_optioned(self.dependencies),
            description: self.description,
            enum_: crate::OptionableConvert::into_optioned(self.enum_),
            example: crate::OptionableConvert::into_optioned(self.example),
            exclusive_maximum: self.exclusive_maximum,
            exclusive_minimum: self.exclusive_minimum,
            external_docs: crate::OptionableConvert::into_optioned(self.external_docs),
            format: self.format,
            id: self.id,
            items: crate::OptionableConvert::into_optioned(self.items),
            max_items: self.max_items,
            max_length: self.max_length,
            max_properties: self.max_properties,
            maximum: self.maximum,
            min_items: self.min_items,
            min_length: self.min_length,
            min_properties: self.min_properties,
            minimum: self.minimum,
            multiple_of: self.multiple_of,
            not: crate::OptionableConvert::into_optioned(self.not),
            nullable: self.nullable,
            one_of: crate::OptionableConvert::into_optioned(self.one_of),
            pattern: self.pattern,
            pattern_properties: crate::OptionableConvert::into_optioned(
                self.pattern_properties,
            ),
            properties: crate::OptionableConvert::into_optioned(self.properties),
            required: self.required,
            title: self.title,
            type_: self.type_,
            unique_items: self.unique_items,
            x_kubernetes_embedded_resource: self.x_kubernetes_embedded_resource,
            x_kubernetes_int_or_string: self.x_kubernetes_int_or_string,
            x_kubernetes_list_map_keys: self.x_kubernetes_list_map_keys,
            x_kubernetes_list_type: self.x_kubernetes_list_type,
            x_kubernetes_map_type: self.x_kubernetes_map_type,
            x_kubernetes_preserve_unknown_fields: self
                .x_kubernetes_preserve_unknown_fields,
            x_kubernetes_validations: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_validations,
            ),
        }
    }
    fn try_from_optioned(value: JSONSchemaPropsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ref_path: value.ref_path,
            schema: value.schema,
            additional_items: crate::OptionableConvert::try_from_optioned(
                value.additional_items,
            )?,
            additional_properties: crate::OptionableConvert::try_from_optioned(
                value.additional_properties,
            )?,
            all_of: crate::OptionableConvert::try_from_optioned(value.all_of)?,
            any_of: crate::OptionableConvert::try_from_optioned(value.any_of)?,
            default: crate::OptionableConvert::try_from_optioned(value.default)?,
            definitions: crate::OptionableConvert::try_from_optioned(value.definitions)?,
            dependencies: crate::OptionableConvert::try_from_optioned(
                value.dependencies,
            )?,
            description: value.description,
            enum_: crate::OptionableConvert::try_from_optioned(value.enum_)?,
            example: crate::OptionableConvert::try_from_optioned(value.example)?,
            exclusive_maximum: value.exclusive_maximum,
            exclusive_minimum: value.exclusive_minimum,
            external_docs: crate::OptionableConvert::try_from_optioned(
                value.external_docs,
            )?,
            format: value.format,
            id: value.id,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            max_items: value.max_items,
            max_length: value.max_length,
            max_properties: value.max_properties,
            maximum: value.maximum,
            min_items: value.min_items,
            min_length: value.min_length,
            min_properties: value.min_properties,
            minimum: value.minimum,
            multiple_of: value.multiple_of,
            not: crate::OptionableConvert::try_from_optioned(value.not)?,
            nullable: value.nullable,
            one_of: crate::OptionableConvert::try_from_optioned(value.one_of)?,
            pattern: value.pattern,
            pattern_properties: crate::OptionableConvert::try_from_optioned(
                value.pattern_properties,
            )?,
            properties: crate::OptionableConvert::try_from_optioned(value.properties)?,
            required: value.required,
            title: value.title,
            type_: value.type_,
            unique_items: value.unique_items,
            x_kubernetes_embedded_resource: value.x_kubernetes_embedded_resource,
            x_kubernetes_int_or_string: value.x_kubernetes_int_or_string,
            x_kubernetes_list_map_keys: value.x_kubernetes_list_map_keys,
            x_kubernetes_list_type: value.x_kubernetes_list_type,
            x_kubernetes_map_type: value.x_kubernetes_map_type,
            x_kubernetes_preserve_unknown_fields: value
                .x_kubernetes_preserve_unknown_fields,
            x_kubernetes_validations: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_validations,
            )?,
        })
    }
    fn merge(&mut self, other: JSONSchemaPropsAc) -> Result<(), crate::Error> {
        self.ref_path = other.ref_path;
        self.schema = other.schema;
        crate::OptionableConvert::merge(
            &mut self.additional_items,
            other.additional_items,
        )?;
        crate::OptionableConvert::merge(
            &mut self.additional_properties,
            other.additional_properties,
        )?;
        crate::OptionableConvert::merge(&mut self.all_of, other.all_of)?;
        crate::OptionableConvert::merge(&mut self.any_of, other.any_of)?;
        crate::OptionableConvert::merge(&mut self.default, other.default)?;
        crate::OptionableConvert::merge(&mut self.definitions, other.definitions)?;
        crate::OptionableConvert::merge(&mut self.dependencies, other.dependencies)?;
        self.description = other.description;
        crate::OptionableConvert::merge(&mut self.enum_, other.enum_)?;
        crate::OptionableConvert::merge(&mut self.example, other.example)?;
        self.exclusive_maximum = other.exclusive_maximum;
        self.exclusive_minimum = other.exclusive_minimum;
        crate::OptionableConvert::merge(&mut self.external_docs, other.external_docs)?;
        self.format = other.format;
        self.id = other.id;
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        self.max_items = other.max_items;
        self.max_length = other.max_length;
        self.max_properties = other.max_properties;
        self.maximum = other.maximum;
        self.min_items = other.min_items;
        self.min_length = other.min_length;
        self.min_properties = other.min_properties;
        self.minimum = other.minimum;
        self.multiple_of = other.multiple_of;
        crate::OptionableConvert::merge(&mut self.not, other.not)?;
        self.nullable = other.nullable;
        crate::OptionableConvert::merge(&mut self.one_of, other.one_of)?;
        self.pattern = other.pattern;
        crate::OptionableConvert::merge(
            &mut self.pattern_properties,
            other.pattern_properties,
        )?;
        crate::OptionableConvert::merge(&mut self.properties, other.properties)?;
        self.required = other.required;
        self.title = other.title;
        self.type_ = other.type_;
        self.unique_items = other.unique_items;
        self.x_kubernetes_embedded_resource = other.x_kubernetes_embedded_resource;
        self.x_kubernetes_int_or_string = other.x_kubernetes_int_or_string;
        self.x_kubernetes_list_map_keys = other.x_kubernetes_list_map_keys;
        self.x_kubernetes_list_type = other.x_kubernetes_list_type;
        self.x_kubernetes_map_type = other.x_kubernetes_map_type;
        self.x_kubernetes_preserve_unknown_fields = other
            .x_kubernetes_preserve_unknown_fields;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_validations,
            other.x_kubernetes_validations,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
> for JSONSchemaPropsAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
