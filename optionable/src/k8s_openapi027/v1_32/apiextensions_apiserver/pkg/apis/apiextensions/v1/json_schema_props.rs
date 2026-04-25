#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
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
    /// default is a default value for undefined object fields. Defaulting is a beta feature under the CustomResourceDefaulting feature gate. Defaulting requires spec.preserveUnknownFields to be false.
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
    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:
    ///
    /// - bsonobjectid: a bson object ID, i.e. a 24 characters hex string - uri: an URI as parsed by Golang net/url.ParseRequestURI - email: an email address as parsed by Golang net/mail.ParseAddress - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 \[RFC1034\]. - ipv4: an IPv4 IP as parsed by Golang net.ParseIP - ipv6: an IPv6 IP as parsed by Golang net.ParseIP - cidr: a CIDR as parsed by Golang net.ParseCIDR - mac: a MAC address as parsed by Golang net.ParseMAC - uuid: an UUID that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?3\[0-9a-f\]{3}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$ - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?4\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?5\[0-9a-f\]{3}-?\[89ab\]\[0-9a-f\]{3}-?\[0-9a-f\]{12}$ - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041" - isbn10: an ISBN10 number string like "0321751043" - isbn13: an ISBN13 number string like "978-0321751041" - creditcard: a credit card number defined by the regex ^(?:4\[0-9\]{12}(?:\[0-9\]{3})?|5\[1-5\]\[0-9\]{14}|6(?:011|5\[0-9\]\[0-9\])\[0-9\]{12}|3\[47\]\[0-9\]{13}|3(?:0\[0-5\]|\[68\]\[0-9\])\[0-9\]{11}|(?:2131|1800|35\\\\d{3})\\\\d{11})$ with any non digit characters mixed in - ssn: a U.S. social security number following the regex ^\\\\d{3}\[- \]?\\\\d{2}\[- \]?\\\\d{4}$ - hexcolor: an hexadecimal color code like "#FFFFFF: following the regex ^#?(\[0-9a-fA-F\]{3}|\[0-9a-fA-F\]{6})$ - rgbcolor: an RGB color code like rgb like "rgb(255,255,2559" - byte: base64 encoded binary data - password: any kind of string - date: a date string like "2006-01-02" as defined by full-date in RFC3339 - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
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
    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_embedded_resource: Option<bool>,
    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///    - type: integer
    ///    - type: string
    /// 2) allOf:
    ///    - anyOf:
    ///      - type: integer
    ///      - type: string
    ///    - ... zero or more
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_int_or_string: Option<bool>,
    /// x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used as the index of the map.
    ///
    /// This tag MUST only be used on lists that have the "x-kubernetes-list-type" extension set to "map". Also, the values specified for this attribute must be a scalar typed field of the child structure (no nesting is supported).
    ///
    /// The properties specified must either be required or have a default value, to ensure those properties are present for all list items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_list_map_keys: Option<std::vec::Vec<std::string::String>>,
    /// x-kubernetes-list-type annotates an array to further describe its topology. This extension must only be used on lists and may have 3 possible values:
    ///
    /// 1) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic lists will be entirely replaced when updated. This extension
    ///      may be used on any type of list (struct, scalar, ...).
    /// 2) `set`:
    ///      Sets are lists that must not have multiple items with the same value. Each
    ///      value must be a scalar, an object with x-kubernetes-map-type `atomic` or an
    ///      array with x-kubernetes-list-type `atomic`.
    /// 3) `map`:
    ///      These lists are like maps in that their elements have a non-index key
    ///      used to identify them. Order is preserved upon merge. The map tag
    ///      must only be used on a list with elements of type object.
    /// Defaults to atomic for arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_list_type: Option<std::string::String>,
    /// x-kubernetes-map-type annotates an object to further describe its topology. This extension must only be used when type is object and may have 2 possible values:
    ///
    /// 1) `granular`:
    ///      These maps are actual maps (key-value pairs) and each fields are independent
    ///      from each other (they can each be manipulated by separate actors). This is
    ///      the default behaviour for all maps.
    /// 2) `atomic`: the list is treated as a single entity, like a scalar.
    ///      Atomic maps will be entirely replaced when updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_map_type: Option<std::string::String>,
    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
    /// x-kubernetes-validations describes a list of validation rules written in the CEL expression language.
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
        if self.ref_path.is_none() {
            self.ref_path = crate::OptionableConvert::try_from_optioned(other.ref_path)?;
        } else if let Some(self_value) = self.ref_path.as_mut()
            && let Some(other_value) = other.ref_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.schema.is_none() {
            self.schema = crate::OptionableConvert::try_from_optioned(other.schema)?;
        } else if let Some(self_value) = self.schema.as_mut()
            && let Some(other_value) = other.schema
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.additional_items.is_none() {
            self.additional_items = crate::OptionableConvert::try_from_optioned(
                other.additional_items,
            )?;
        } else if let Some(self_value) = self.additional_items.as_mut()
            && let Some(other_value) = other.additional_items
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.additional_properties.is_none() {
            self.additional_properties = crate::OptionableConvert::try_from_optioned(
                other.additional_properties,
            )?;
        } else if let Some(self_value) = self.additional_properties.as_mut()
            && let Some(other_value) = other.additional_properties
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.all_of.is_none() {
            self.all_of = crate::OptionableConvert::try_from_optioned(other.all_of)?;
        } else if let Some(self_value) = self.all_of.as_mut()
            && let Some(other_value) = other.all_of
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.any_of.is_none() {
            self.any_of = crate::OptionableConvert::try_from_optioned(other.any_of)?;
        } else if let Some(self_value) = self.any_of.as_mut()
            && let Some(other_value) = other.any_of
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.default.is_none() {
            self.default = crate::OptionableConvert::try_from_optioned(other.default)?;
        } else if let Some(self_value) = self.default.as_mut()
            && let Some(other_value) = other.default
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.definitions.is_none() {
            self.definitions = crate::OptionableConvert::try_from_optioned(
                other.definitions,
            )?;
        } else if let Some(self_value) = self.definitions.as_mut()
            && let Some(other_value) = other.definitions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.dependencies.is_none() {
            self.dependencies = crate::OptionableConvert::try_from_optioned(
                other.dependencies,
            )?;
        } else if let Some(self_value) = self.dependencies.as_mut()
            && let Some(other_value) = other.dependencies
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.description.is_none() {
            self.description = crate::OptionableConvert::try_from_optioned(
                other.description,
            )?;
        } else if let Some(self_value) = self.description.as_mut()
            && let Some(other_value) = other.description
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.enum_.is_none() {
            self.enum_ = crate::OptionableConvert::try_from_optioned(other.enum_)?;
        } else if let Some(self_value) = self.enum_.as_mut()
            && let Some(other_value) = other.enum_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.example.is_none() {
            self.example = crate::OptionableConvert::try_from_optioned(other.example)?;
        } else if let Some(self_value) = self.example.as_mut()
            && let Some(other_value) = other.example
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.exclusive_maximum.is_none() {
            self.exclusive_maximum = crate::OptionableConvert::try_from_optioned(
                other.exclusive_maximum,
            )?;
        } else if let Some(self_value) = self.exclusive_maximum.as_mut()
            && let Some(other_value) = other.exclusive_maximum
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.exclusive_minimum.is_none() {
            self.exclusive_minimum = crate::OptionableConvert::try_from_optioned(
                other.exclusive_minimum,
            )?;
        } else if let Some(self_value) = self.exclusive_minimum.as_mut()
            && let Some(other_value) = other.exclusive_minimum
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.external_docs.is_none() {
            self.external_docs = crate::OptionableConvert::try_from_optioned(
                other.external_docs,
            )?;
        } else if let Some(self_value) = self.external_docs.as_mut()
            && let Some(other_value) = other.external_docs
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.format.is_none() {
            self.format = crate::OptionableConvert::try_from_optioned(other.format)?;
        } else if let Some(self_value) = self.format.as_mut()
            && let Some(other_value) = other.format
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.id.is_none() {
            self.id = crate::OptionableConvert::try_from_optioned(other.id)?;
        } else if let Some(self_value) = self.id.as_mut()
            && let Some(other_value) = other.id
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.items.is_none() {
            self.items = crate::OptionableConvert::try_from_optioned(other.items)?;
        } else if let Some(self_value) = self.items.as_mut()
            && let Some(other_value) = other.items
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.max_items.is_none() {
            self.max_items = crate::OptionableConvert::try_from_optioned(
                other.max_items,
            )?;
        } else if let Some(self_value) = self.max_items.as_mut()
            && let Some(other_value) = other.max_items
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.max_length.is_none() {
            self.max_length = crate::OptionableConvert::try_from_optioned(
                other.max_length,
            )?;
        } else if let Some(self_value) = self.max_length.as_mut()
            && let Some(other_value) = other.max_length
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.max_properties.is_none() {
            self.max_properties = crate::OptionableConvert::try_from_optioned(
                other.max_properties,
            )?;
        } else if let Some(self_value) = self.max_properties.as_mut()
            && let Some(other_value) = other.max_properties
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.maximum.is_none() {
            self.maximum = crate::OptionableConvert::try_from_optioned(other.maximum)?;
        } else if let Some(self_value) = self.maximum.as_mut()
            && let Some(other_value) = other.maximum
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.min_items.is_none() {
            self.min_items = crate::OptionableConvert::try_from_optioned(
                other.min_items,
            )?;
        } else if let Some(self_value) = self.min_items.as_mut()
            && let Some(other_value) = other.min_items
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.min_length.is_none() {
            self.min_length = crate::OptionableConvert::try_from_optioned(
                other.min_length,
            )?;
        } else if let Some(self_value) = self.min_length.as_mut()
            && let Some(other_value) = other.min_length
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.min_properties.is_none() {
            self.min_properties = crate::OptionableConvert::try_from_optioned(
                other.min_properties,
            )?;
        } else if let Some(self_value) = self.min_properties.as_mut()
            && let Some(other_value) = other.min_properties
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.minimum.is_none() {
            self.minimum = crate::OptionableConvert::try_from_optioned(other.minimum)?;
        } else if let Some(self_value) = self.minimum.as_mut()
            && let Some(other_value) = other.minimum
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.multiple_of.is_none() {
            self.multiple_of = crate::OptionableConvert::try_from_optioned(
                other.multiple_of,
            )?;
        } else if let Some(self_value) = self.multiple_of.as_mut()
            && let Some(other_value) = other.multiple_of
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.not.is_none() {
            self.not = crate::OptionableConvert::try_from_optioned(other.not)?;
        } else if let Some(self_value) = self.not.as_mut()
            && let Some(other_value) = other.not
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.nullable.is_none() {
            self.nullable = crate::OptionableConvert::try_from_optioned(other.nullable)?;
        } else if let Some(self_value) = self.nullable.as_mut()
            && let Some(other_value) = other.nullable
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.one_of.is_none() {
            self.one_of = crate::OptionableConvert::try_from_optioned(other.one_of)?;
        } else if let Some(self_value) = self.one_of.as_mut()
            && let Some(other_value) = other.one_of
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pattern.is_none() {
            self.pattern = crate::OptionableConvert::try_from_optioned(other.pattern)?;
        } else if let Some(self_value) = self.pattern.as_mut()
            && let Some(other_value) = other.pattern
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pattern_properties.is_none() {
            self.pattern_properties = crate::OptionableConvert::try_from_optioned(
                other.pattern_properties,
            )?;
        } else if let Some(self_value) = self.pattern_properties.as_mut()
            && let Some(other_value) = other.pattern_properties
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.properties.is_none() {
            self.properties = crate::OptionableConvert::try_from_optioned(
                other.properties,
            )?;
        } else if let Some(self_value) = self.properties.as_mut()
            && let Some(other_value) = other.properties
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.required.is_none() {
            self.required = crate::OptionableConvert::try_from_optioned(other.required)?;
        } else if let Some(self_value) = self.required.as_mut()
            && let Some(other_value) = other.required
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.title.is_none() {
            self.title = crate::OptionableConvert::try_from_optioned(other.title)?;
        } else if let Some(self_value) = self.title.as_mut()
            && let Some(other_value) = other.title
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.unique_items.is_none() {
            self.unique_items = crate::OptionableConvert::try_from_optioned(
                other.unique_items,
            )?;
        } else if let Some(self_value) = self.unique_items.as_mut()
            && let Some(other_value) = other.unique_items
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_embedded_resource.is_none() {
            self.x_kubernetes_embedded_resource = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_embedded_resource,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_embedded_resource.as_mut()
            && let Some(other_value) = other.x_kubernetes_embedded_resource
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_int_or_string.is_none() {
            self.x_kubernetes_int_or_string = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_int_or_string,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_int_or_string.as_mut()
            && let Some(other_value) = other.x_kubernetes_int_or_string
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_list_map_keys.is_none() {
            self.x_kubernetes_list_map_keys = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_list_map_keys,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_list_map_keys.as_mut()
            && let Some(other_value) = other.x_kubernetes_list_map_keys
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_list_type.is_none() {
            self.x_kubernetes_list_type = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_list_type,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_list_type.as_mut()
            && let Some(other_value) = other.x_kubernetes_list_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_map_type.is_none() {
            self.x_kubernetes_map_type = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_map_type,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_map_type.as_mut()
            && let Some(other_value) = other.x_kubernetes_map_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_preserve_unknown_fields.is_none() {
            self.x_kubernetes_preserve_unknown_fields = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_preserve_unknown_fields,
            )?;
        } else if let Some(self_value) = self
            .x_kubernetes_preserve_unknown_fields
            .as_mut()
            && let Some(other_value) = other.x_kubernetes_preserve_unknown_fields
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.x_kubernetes_validations.is_none() {
            self.x_kubernetes_validations = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_validations,
            )?;
        } else if let Some(self_value) = self.x_kubernetes_validations.as_mut()
            && let Some(other_value) = other.x_kubernetes_validations
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for JSONSchemaPropsAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.ref_path, other.ref_path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.schema, other.schema);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.additional_items,
            other.additional_items,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.additional_properties,
            other.additional_properties,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.all_of, other.all_of);
        k8s_openapi027::DeepMerge::merge_from(&mut self.any_of, other.any_of);
        k8s_openapi027::DeepMerge::merge_from(&mut self.default, other.default);
        k8s_openapi027::DeepMerge::merge_from(&mut self.definitions, other.definitions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.dependencies,
            other.dependencies,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.description, other.description);
        k8s_openapi027::DeepMerge::merge_from(&mut self.enum_, other.enum_);
        k8s_openapi027::DeepMerge::merge_from(&mut self.example, other.example);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.exclusive_maximum,
            other.exclusive_maximum,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.exclusive_minimum,
            other.exclusive_minimum,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.external_docs,
            other.external_docs,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.format, other.format);
        k8s_openapi027::DeepMerge::merge_from(&mut self.id, other.id);
        k8s_openapi027::DeepMerge::merge_from(&mut self.items, other.items);
        k8s_openapi027::DeepMerge::merge_from(&mut self.max_items, other.max_items);
        k8s_openapi027::DeepMerge::merge_from(&mut self.max_length, other.max_length);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.max_properties,
            other.max_properties,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.maximum, other.maximum);
        k8s_openapi027::DeepMerge::merge_from(&mut self.min_items, other.min_items);
        k8s_openapi027::DeepMerge::merge_from(&mut self.min_length, other.min_length);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.min_properties,
            other.min_properties,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.minimum, other.minimum);
        k8s_openapi027::DeepMerge::merge_from(&mut self.multiple_of, other.multiple_of);
        k8s_openapi027::DeepMerge::merge_from(&mut self.not, other.not);
        k8s_openapi027::DeepMerge::merge_from(&mut self.nullable, other.nullable);
        k8s_openapi027::DeepMerge::merge_from(&mut self.one_of, other.one_of);
        k8s_openapi027::DeepMerge::merge_from(&mut self.pattern, other.pattern);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.pattern_properties,
            other.pattern_properties,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.properties, other.properties);
        k8s_openapi027::DeepMerge::merge_from(&mut self.required, other.required);
        k8s_openapi027::DeepMerge::merge_from(&mut self.title, other.title);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.unique_items,
            other.unique_items,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_embedded_resource,
            other.x_kubernetes_embedded_resource,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_int_or_string,
            other.x_kubernetes_int_or_string,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_list_map_keys,
            other.x_kubernetes_list_map_keys,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_list_type,
            other.x_kubernetes_list_type,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_map_type,
            other.x_kubernetes_map_type,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_preserve_unknown_fields,
            other.x_kubernetes_preserve_unknown_fields,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.x_kubernetes_validations,
            other.x_kubernetes_validations,
        );
    }
}
