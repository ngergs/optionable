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
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::json_schema_props::JSONSchemaProps {
    type Optioned = JSONSchemaPropsOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONSchemaPropsOpt {
    type Optioned = JSONSchemaPropsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::json_schema_props::JSONSchemaProps {
    fn into_optioned(self) -> JSONSchemaPropsOpt {
        JSONSchemaPropsOpt {
            ref_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ref_path),
            schema: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.schema),
            additional_items: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
            > as crate::OptionableConvert>::into_optioned(self.additional_items),
            additional_properties: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
            > as crate::OptionableConvert>::into_optioned(self.additional_properties),
            all_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.all_of),
            any_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.any_of),
            default: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
            > as crate::OptionableConvert>::into_optioned(self.default),
            definitions: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.definitions),
            dependencies: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
                >,
            > as crate::OptionableConvert>::into_optioned(self.dependencies),
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.description),
            enum_: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
                >,
            > as crate::OptionableConvert>::into_optioned(self.enum_),
            example: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
            > as crate::OptionableConvert>::into_optioned(self.example),
            exclusive_maximum: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.exclusive_maximum),
            exclusive_minimum: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.exclusive_minimum),
            external_docs: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
            > as crate::OptionableConvert>::into_optioned(self.external_docs),
            format: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.format),
            id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.id),
            items: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray,
            > as crate::OptionableConvert>::into_optioned(self.items),
            max_items: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.max_items),
            max_length: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.max_length),
            max_properties: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.max_properties),
            maximum: <Option<
                f64,
            > as crate::OptionableConvert>::into_optioned(self.maximum),
            min_items: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.min_items),
            min_length: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.min_length),
            min_properties: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.min_properties),
            minimum: <Option<
                f64,
            > as crate::OptionableConvert>::into_optioned(self.minimum),
            multiple_of: <Option<
                f64,
            > as crate::OptionableConvert>::into_optioned(self.multiple_of),
            not: <Option<
                std::boxed::Box<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.not),
            nullable: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.nullable),
            one_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.one_of),
            pattern: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.pattern),
            pattern_properties: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.pattern_properties),
            properties: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::into_optioned(self.properties),
            required: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.required),
            title: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.title),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
            unique_items: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.unique_items),
            x_kubernetes_embedded_resource: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.x_kubernetes_embedded_resource,
            ),
            x_kubernetes_int_or_string: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.x_kubernetes_int_or_string,
            ),
            x_kubernetes_list_map_keys: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(
                self.x_kubernetes_list_map_keys,
            ),
            x_kubernetes_list_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.x_kubernetes_list_type),
            x_kubernetes_map_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.x_kubernetes_map_type),
            x_kubernetes_preserve_unknown_fields: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.x_kubernetes_preserve_unknown_fields,
            ),
            x_kubernetes_validations: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
                >,
            > as crate::OptionableConvert>::into_optioned(self.x_kubernetes_validations),
        }
    }
    fn try_from_optioned(
        value: JSONSchemaPropsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ref_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ref_path)?,
            schema: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.schema)?,
            additional_items: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
            > as crate::OptionableConvert>::try_from_optioned(value.additional_items)?,
            additional_properties: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.additional_properties,
            )?,
            all_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.all_of)?,
            any_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.any_of)?,
            default: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
            > as crate::OptionableConvert>::try_from_optioned(value.default)?,
            definitions: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.definitions)?,
            dependencies: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.dependencies)?,
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.description)?,
            enum_: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.enum_)?,
            example: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
            > as crate::OptionableConvert>::try_from_optioned(value.example)?,
            exclusive_maximum: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.exclusive_maximum)?,
            exclusive_minimum: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.exclusive_minimum)?,
            external_docs: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
            > as crate::OptionableConvert>::try_from_optioned(value.external_docs)?,
            format: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.format)?,
            id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.id)?,
            items: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray,
            > as crate::OptionableConvert>::try_from_optioned(value.items)?,
            max_items: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.max_items)?,
            max_length: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.max_length)?,
            max_properties: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.max_properties)?,
            maximum: <Option<
                f64,
            > as crate::OptionableConvert>::try_from_optioned(value.maximum)?,
            min_items: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.min_items)?,
            min_length: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.min_length)?,
            min_properties: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.min_properties)?,
            minimum: <Option<
                f64,
            > as crate::OptionableConvert>::try_from_optioned(value.minimum)?,
            multiple_of: <Option<
                f64,
            > as crate::OptionableConvert>::try_from_optioned(value.multiple_of)?,
            not: <Option<
                std::boxed::Box<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.not)?,
            nullable: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.nullable)?,
            one_of: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.one_of)?,
            pattern: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.pattern)?,
            pattern_properties: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.pattern_properties)?,
            properties: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.properties)?,
            required: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.required)?,
            title: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.title)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
            unique_items: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.unique_items)?,
            x_kubernetes_embedded_resource: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_embedded_resource,
            )?,
            x_kubernetes_int_or_string: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_int_or_string,
            )?,
            x_kubernetes_list_map_keys: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_list_map_keys,
            )?,
            x_kubernetes_list_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_list_type,
            )?,
            x_kubernetes_map_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_map_type,
            )?,
            x_kubernetes_preserve_unknown_fields: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_preserve_unknown_fields,
            )?,
            x_kubernetes_validations: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.x_kubernetes_validations,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: JSONSchemaPropsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.ref_path, other.ref_path)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.schema, other.schema)?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
        > as crate::OptionableConvert>::merge(
            &mut self.additional_items,
            other.additional_items,
        )?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrBool,
        > as crate::OptionableConvert>::merge(
            &mut self.additional_properties,
            other.additional_properties,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.all_of, other.all_of)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.any_of, other.any_of)?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
        > as crate::OptionableConvert>::merge(&mut self.default, other.default)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.definitions, other.definitions)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrStringArray,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.dependencies,
            other.dependencies,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.description, other.description)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
            >,
        > as crate::OptionableConvert>::merge(&mut self.enum_, other.enum_)?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSON,
        > as crate::OptionableConvert>::merge(&mut self.example, other.example)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.exclusive_maximum,
            other.exclusive_maximum,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.exclusive_minimum,
            other.exclusive_minimum,
        )?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
        > as crate::OptionableConvert>::merge(
            &mut self.external_docs,
            other.external_docs,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.format, other.format)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.id, other.id)?;
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray,
        > as crate::OptionableConvert>::merge(&mut self.items, other.items)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.max_items, other.max_items)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.max_length, other.max_length)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.max_properties,
            other.max_properties,
        )?;
        <Option<
            f64,
        > as crate::OptionableConvert>::merge(&mut self.maximum, other.maximum)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.min_items, other.min_items)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.min_length, other.min_length)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.min_properties,
            other.min_properties,
        )?;
        <Option<
            f64,
        > as crate::OptionableConvert>::merge(&mut self.minimum, other.minimum)?;
        <Option<
            f64,
        > as crate::OptionableConvert>::merge(&mut self.multiple_of, other.multiple_of)?;
        <Option<
            std::boxed::Box<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.not, other.not)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.nullable, other.nullable)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.one_of, other.one_of)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.pattern, other.pattern)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.pattern_properties,
            other.pattern_properties,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
            >,
        > as crate::OptionableConvert>::merge(&mut self.properties, other.properties)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.required, other.required)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.title, other.title)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.unique_items,
            other.unique_items,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_embedded_resource,
            other.x_kubernetes_embedded_resource,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_int_or_string,
            other.x_kubernetes_int_or_string,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_list_map_keys,
            other.x_kubernetes_list_map_keys,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_list_type,
            other.x_kubernetes_list_type,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_map_type,
            other.x_kubernetes_map_type,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_preserve_unknown_fields,
            other.x_kubernetes_preserve_unknown_fields,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ValidationRule,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.x_kubernetes_validations,
            other.x_kubernetes_validations,
        )?;
        Ok(())
    }
}
