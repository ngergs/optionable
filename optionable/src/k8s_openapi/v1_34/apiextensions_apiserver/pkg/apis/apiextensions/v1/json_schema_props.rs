pub struct JSONSchemaPropsOpt {
    pub ref_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub schema: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub additional_items: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
    > as crate::Optionable>::Optioned,
    pub additional_properties: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
    > as crate::Optionable>::Optioned,
    pub all_of: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub any_of: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub default: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
    > as crate::Optionable>::Optioned,
    pub definitions: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub dependencies: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
        >,
    > as crate::Optionable>::Optioned,
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub enum_: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
        >,
    > as crate::Optionable>::Optioned,
    pub example: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
    > as crate::Optionable>::Optioned,
    pub exclusive_maximum: <Option<bool> as crate::Optionable>::Optioned,
    pub exclusive_minimum: <Option<bool> as crate::Optionable>::Optioned,
    pub external_docs: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
    > as crate::Optionable>::Optioned,
    pub format: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub items: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray,
    > as crate::Optionable>::Optioned,
    pub max_items: <Option<i64> as crate::Optionable>::Optioned,
    pub max_length: <Option<i64> as crate::Optionable>::Optioned,
    pub max_properties: <Option<i64> as crate::Optionable>::Optioned,
    pub maximum: <Option<f64> as crate::Optionable>::Optioned,
    pub min_items: <Option<i64> as crate::Optionable>::Optioned,
    pub min_length: <Option<i64> as crate::Optionable>::Optioned,
    pub min_properties: <Option<i64> as crate::Optionable>::Optioned,
    pub minimum: <Option<f64> as crate::Optionable>::Optioned,
    pub multiple_of: <Option<f64> as crate::Optionable>::Optioned,
    pub not: <Option<
        std::boxed::Box<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub nullable: <Option<bool> as crate::Optionable>::Optioned,
    pub one_of: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub pattern: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pattern_properties: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub properties: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
        >,
    > as crate::Optionable>::Optioned,
    pub required: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub title: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub unique_items: <Option<bool> as crate::Optionable>::Optioned,
    pub x_kubernetes_embedded_resource: <Option<bool> as crate::Optionable>::Optioned,
    pub x_kubernetes_int_or_string: <Option<bool> as crate::Optionable>::Optioned,
    pub x_kubernetes_list_map_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub x_kubernetes_list_type: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub x_kubernetes_map_type: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub x_kubernetes_preserve_unknown_fields: <Option<
        bool,
    > as crate::Optionable>::Optioned,
    pub x_kubernetes_validations: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps {
    type Optioned = JSONSchemaPropsOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOpt {
    type Optioned = JSONSchemaPropsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps {
    fn into_optioned(self) -> JSONSchemaPropsOpt {
        JSONSchemaPropsOpt {
            ref_path: crate::OptionableConvert::into_optioned(self.ref_path),
            schema: crate::OptionableConvert::into_optioned(self.schema),
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
            description: crate::OptionableConvert::into_optioned(self.description),
            enum_: crate::OptionableConvert::into_optioned(self.enum_),
            example: crate::OptionableConvert::into_optioned(self.example),
            exclusive_maximum: crate::OptionableConvert::into_optioned(
                self.exclusive_maximum,
            ),
            exclusive_minimum: crate::OptionableConvert::into_optioned(
                self.exclusive_minimum,
            ),
            external_docs: crate::OptionableConvert::into_optioned(self.external_docs),
            format: crate::OptionableConvert::into_optioned(self.format),
            id: crate::OptionableConvert::into_optioned(self.id),
            items: crate::OptionableConvert::into_optioned(self.items),
            max_items: crate::OptionableConvert::into_optioned(self.max_items),
            max_length: crate::OptionableConvert::into_optioned(self.max_length),
            max_properties: crate::OptionableConvert::into_optioned(self.max_properties),
            maximum: crate::OptionableConvert::into_optioned(self.maximum),
            min_items: crate::OptionableConvert::into_optioned(self.min_items),
            min_length: crate::OptionableConvert::into_optioned(self.min_length),
            min_properties: crate::OptionableConvert::into_optioned(self.min_properties),
            minimum: crate::OptionableConvert::into_optioned(self.minimum),
            multiple_of: crate::OptionableConvert::into_optioned(self.multiple_of),
            not: crate::OptionableConvert::into_optioned(self.not),
            nullable: crate::OptionableConvert::into_optioned(self.nullable),
            one_of: crate::OptionableConvert::into_optioned(self.one_of),
            pattern: crate::OptionableConvert::into_optioned(self.pattern),
            pattern_properties: crate::OptionableConvert::into_optioned(
                self.pattern_properties,
            ),
            properties: crate::OptionableConvert::into_optioned(self.properties),
            required: crate::OptionableConvert::into_optioned(self.required),
            title: crate::OptionableConvert::into_optioned(self.title),
            type_: crate::OptionableConvert::into_optioned(self.type_),
            unique_items: crate::OptionableConvert::into_optioned(self.unique_items),
            x_kubernetes_embedded_resource: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_embedded_resource,
            ),
            x_kubernetes_int_or_string: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_int_or_string,
            ),
            x_kubernetes_list_map_keys: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_list_map_keys,
            ),
            x_kubernetes_list_type: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_list_type,
            ),
            x_kubernetes_map_type: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_map_type,
            ),
            x_kubernetes_preserve_unknown_fields: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_preserve_unknown_fields,
            ),
            x_kubernetes_validations: crate::OptionableConvert::into_optioned(
                self.x_kubernetes_validations,
            ),
        }
    }
    fn try_from_optioned(
        value: JSONSchemaPropsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ref_path: crate::OptionableConvert::try_from_optioned(value.ref_path)?,
            schema: crate::OptionableConvert::try_from_optioned(value.schema)?,
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
            description: crate::OptionableConvert::try_from_optioned(value.description)?,
            enum_: crate::OptionableConvert::try_from_optioned(value.enum_)?,
            example: crate::OptionableConvert::try_from_optioned(value.example)?,
            exclusive_maximum: crate::OptionableConvert::try_from_optioned(
                value.exclusive_maximum,
            )?,
            exclusive_minimum: crate::OptionableConvert::try_from_optioned(
                value.exclusive_minimum,
            )?,
            external_docs: crate::OptionableConvert::try_from_optioned(
                value.external_docs,
            )?,
            format: crate::OptionableConvert::try_from_optioned(value.format)?,
            id: crate::OptionableConvert::try_from_optioned(value.id)?,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            max_items: crate::OptionableConvert::try_from_optioned(value.max_items)?,
            max_length: crate::OptionableConvert::try_from_optioned(value.max_length)?,
            max_properties: crate::OptionableConvert::try_from_optioned(
                value.max_properties,
            )?,
            maximum: crate::OptionableConvert::try_from_optioned(value.maximum)?,
            min_items: crate::OptionableConvert::try_from_optioned(value.min_items)?,
            min_length: crate::OptionableConvert::try_from_optioned(value.min_length)?,
            min_properties: crate::OptionableConvert::try_from_optioned(
                value.min_properties,
            )?,
            minimum: crate::OptionableConvert::try_from_optioned(value.minimum)?,
            multiple_of: crate::OptionableConvert::try_from_optioned(value.multiple_of)?,
            not: crate::OptionableConvert::try_from_optioned(value.not)?,
            nullable: crate::OptionableConvert::try_from_optioned(value.nullable)?,
            one_of: crate::OptionableConvert::try_from_optioned(value.one_of)?,
            pattern: crate::OptionableConvert::try_from_optioned(value.pattern)?,
            pattern_properties: crate::OptionableConvert::try_from_optioned(
                value.pattern_properties,
            )?,
            properties: crate::OptionableConvert::try_from_optioned(value.properties)?,
            required: crate::OptionableConvert::try_from_optioned(value.required)?,
            title: crate::OptionableConvert::try_from_optioned(value.title)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
            unique_items: crate::OptionableConvert::try_from_optioned(
                value.unique_items,
            )?,
            x_kubernetes_embedded_resource: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_embedded_resource,
            )?,
            x_kubernetes_int_or_string: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_int_or_string,
            )?,
            x_kubernetes_list_map_keys: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_list_map_keys,
            )?,
            x_kubernetes_list_type: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_list_type,
            )?,
            x_kubernetes_map_type: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_map_type,
            )?,
            x_kubernetes_preserve_unknown_fields: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_preserve_unknown_fields,
            )?,
            x_kubernetes_validations: crate::OptionableConvert::try_from_optioned(
                value.x_kubernetes_validations,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: JSONSchemaPropsOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.ref_path, other.ref_path)?;
        crate::OptionableConvert::merge(&mut self.schema, other.schema)?;
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
        crate::OptionableConvert::merge(&mut self.description, other.description)?;
        crate::OptionableConvert::merge(&mut self.enum_, other.enum_)?;
        crate::OptionableConvert::merge(&mut self.example, other.example)?;
        crate::OptionableConvert::merge(
            &mut self.exclusive_maximum,
            other.exclusive_maximum,
        )?;
        crate::OptionableConvert::merge(
            &mut self.exclusive_minimum,
            other.exclusive_minimum,
        )?;
        crate::OptionableConvert::merge(&mut self.external_docs, other.external_docs)?;
        crate::OptionableConvert::merge(&mut self.format, other.format)?;
        crate::OptionableConvert::merge(&mut self.id, other.id)?;
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        crate::OptionableConvert::merge(&mut self.max_items, other.max_items)?;
        crate::OptionableConvert::merge(&mut self.max_length, other.max_length)?;
        crate::OptionableConvert::merge(&mut self.max_properties, other.max_properties)?;
        crate::OptionableConvert::merge(&mut self.maximum, other.maximum)?;
        crate::OptionableConvert::merge(&mut self.min_items, other.min_items)?;
        crate::OptionableConvert::merge(&mut self.min_length, other.min_length)?;
        crate::OptionableConvert::merge(&mut self.min_properties, other.min_properties)?;
        crate::OptionableConvert::merge(&mut self.minimum, other.minimum)?;
        crate::OptionableConvert::merge(&mut self.multiple_of, other.multiple_of)?;
        crate::OptionableConvert::merge(&mut self.not, other.not)?;
        crate::OptionableConvert::merge(&mut self.nullable, other.nullable)?;
        crate::OptionableConvert::merge(&mut self.one_of, other.one_of)?;
        crate::OptionableConvert::merge(&mut self.pattern, other.pattern)?;
        crate::OptionableConvert::merge(
            &mut self.pattern_properties,
            other.pattern_properties,
        )?;
        crate::OptionableConvert::merge(&mut self.properties, other.properties)?;
        crate::OptionableConvert::merge(&mut self.required, other.required)?;
        crate::OptionableConvert::merge(&mut self.title, other.title)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        crate::OptionableConvert::merge(&mut self.unique_items, other.unique_items)?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_embedded_resource,
            other.x_kubernetes_embedded_resource,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_int_or_string,
            other.x_kubernetes_int_or_string,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_list_map_keys,
            other.x_kubernetes_list_map_keys,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_list_type,
            other.x_kubernetes_list_type,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_map_type,
            other.x_kubernetes_map_type,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_preserve_unknown_fields,
            other.x_kubernetes_preserve_unknown_fields,
        )?;
        crate::OptionableConvert::merge(
            &mut self.x_kubernetes_validations,
            other.x_kubernetes_validations,
        )?;
        Ok(())
    }
}
