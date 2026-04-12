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
        } else {
            crate::OptionableConvert::merge(&mut self.ref_path, other.ref_path)?;
        }
        if self.schema.is_none() {
            self.schema = crate::OptionableConvert::try_from_optioned(other.schema)?;
        } else {
            crate::OptionableConvert::merge(&mut self.schema, other.schema)?;
        }
        if self.additional_items.is_none() {
            self.additional_items = crate::OptionableConvert::try_from_optioned(
                other.additional_items,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.additional_items,
                other.additional_items,
            )?;
        }
        if self.additional_properties.is_none() {
            self.additional_properties = crate::OptionableConvert::try_from_optioned(
                other.additional_properties,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.additional_properties,
                other.additional_properties,
            )?;
        }
        if self.all_of.is_none() {
            self.all_of = crate::OptionableConvert::try_from_optioned(other.all_of)?;
        } else {
            crate::OptionableConvert::merge(&mut self.all_of, other.all_of)?;
        }
        if self.any_of.is_none() {
            self.any_of = crate::OptionableConvert::try_from_optioned(other.any_of)?;
        } else {
            crate::OptionableConvert::merge(&mut self.any_of, other.any_of)?;
        }
        if self.default.is_none() {
            self.default = crate::OptionableConvert::try_from_optioned(other.default)?;
        } else {
            crate::OptionableConvert::merge(&mut self.default, other.default)?;
        }
        if self.definitions.is_none() {
            self.definitions = crate::OptionableConvert::try_from_optioned(
                other.definitions,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.definitions, other.definitions)?;
        }
        if self.dependencies.is_none() {
            self.dependencies = crate::OptionableConvert::try_from_optioned(
                other.dependencies,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.dependencies, other.dependencies)?;
        }
        if self.description.is_none() {
            self.description = crate::OptionableConvert::try_from_optioned(
                other.description,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.description, other.description)?;
        }
        if self.enum_.is_none() {
            self.enum_ = crate::OptionableConvert::try_from_optioned(other.enum_)?;
        } else {
            crate::OptionableConvert::merge(&mut self.enum_, other.enum_)?;
        }
        if self.example.is_none() {
            self.example = crate::OptionableConvert::try_from_optioned(other.example)?;
        } else {
            crate::OptionableConvert::merge(&mut self.example, other.example)?;
        }
        if self.exclusive_maximum.is_none() {
            self.exclusive_maximum = crate::OptionableConvert::try_from_optioned(
                other.exclusive_maximum,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.exclusive_maximum,
                other.exclusive_maximum,
            )?;
        }
        if self.exclusive_minimum.is_none() {
            self.exclusive_minimum = crate::OptionableConvert::try_from_optioned(
                other.exclusive_minimum,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.exclusive_minimum,
                other.exclusive_minimum,
            )?;
        }
        if self.external_docs.is_none() {
            self.external_docs = crate::OptionableConvert::try_from_optioned(
                other.external_docs,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.external_docs,
                other.external_docs,
            )?;
        }
        if self.format.is_none() {
            self.format = crate::OptionableConvert::try_from_optioned(other.format)?;
        } else {
            crate::OptionableConvert::merge(&mut self.format, other.format)?;
        }
        if self.id.is_none() {
            self.id = crate::OptionableConvert::try_from_optioned(other.id)?;
        } else {
            crate::OptionableConvert::merge(&mut self.id, other.id)?;
        }
        if self.items.is_none() {
            self.items = crate::OptionableConvert::try_from_optioned(other.items)?;
        } else {
            crate::OptionableConvert::merge(&mut self.items, other.items)?;
        }
        if self.max_items.is_none() {
            self.max_items = crate::OptionableConvert::try_from_optioned(
                other.max_items,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.max_items, other.max_items)?;
        }
        if self.max_length.is_none() {
            self.max_length = crate::OptionableConvert::try_from_optioned(
                other.max_length,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.max_length, other.max_length)?;
        }
        if self.max_properties.is_none() {
            self.max_properties = crate::OptionableConvert::try_from_optioned(
                other.max_properties,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.max_properties,
                other.max_properties,
            )?;
        }
        if self.maximum.is_none() {
            self.maximum = crate::OptionableConvert::try_from_optioned(other.maximum)?;
        } else {
            crate::OptionableConvert::merge(&mut self.maximum, other.maximum)?;
        }
        if self.min_items.is_none() {
            self.min_items = crate::OptionableConvert::try_from_optioned(
                other.min_items,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.min_items, other.min_items)?;
        }
        if self.min_length.is_none() {
            self.min_length = crate::OptionableConvert::try_from_optioned(
                other.min_length,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.min_length, other.min_length)?;
        }
        if self.min_properties.is_none() {
            self.min_properties = crate::OptionableConvert::try_from_optioned(
                other.min_properties,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.min_properties,
                other.min_properties,
            )?;
        }
        if self.minimum.is_none() {
            self.minimum = crate::OptionableConvert::try_from_optioned(other.minimum)?;
        } else {
            crate::OptionableConvert::merge(&mut self.minimum, other.minimum)?;
        }
        if self.multiple_of.is_none() {
            self.multiple_of = crate::OptionableConvert::try_from_optioned(
                other.multiple_of,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.multiple_of, other.multiple_of)?;
        }
        if self.not.is_none() {
            self.not = crate::OptionableConvert::try_from_optioned(other.not)?;
        } else {
            crate::OptionableConvert::merge(&mut self.not, other.not)?;
        }
        if self.nullable.is_none() {
            self.nullable = crate::OptionableConvert::try_from_optioned(other.nullable)?;
        } else {
            crate::OptionableConvert::merge(&mut self.nullable, other.nullable)?;
        }
        if self.one_of.is_none() {
            self.one_of = crate::OptionableConvert::try_from_optioned(other.one_of)?;
        } else {
            crate::OptionableConvert::merge(&mut self.one_of, other.one_of)?;
        }
        if self.pattern.is_none() {
            self.pattern = crate::OptionableConvert::try_from_optioned(other.pattern)?;
        } else {
            crate::OptionableConvert::merge(&mut self.pattern, other.pattern)?;
        }
        if self.pattern_properties.is_none() {
            self.pattern_properties = crate::OptionableConvert::try_from_optioned(
                other.pattern_properties,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.pattern_properties,
                other.pattern_properties,
            )?;
        }
        if self.properties.is_none() {
            self.properties = crate::OptionableConvert::try_from_optioned(
                other.properties,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.properties, other.properties)?;
        }
        if self.required.is_none() {
            self.required = crate::OptionableConvert::try_from_optioned(other.required)?;
        } else {
            crate::OptionableConvert::merge(&mut self.required, other.required)?;
        }
        if self.title.is_none() {
            self.title = crate::OptionableConvert::try_from_optioned(other.title)?;
        } else {
            crate::OptionableConvert::merge(&mut self.title, other.title)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else {
            crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        }
        if self.unique_items.is_none() {
            self.unique_items = crate::OptionableConvert::try_from_optioned(
                other.unique_items,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.unique_items, other.unique_items)?;
        }
        if self.x_kubernetes_embedded_resource.is_none() {
            self.x_kubernetes_embedded_resource = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_embedded_resource,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_embedded_resource,
                other.x_kubernetes_embedded_resource,
            )?;
        }
        if self.x_kubernetes_int_or_string.is_none() {
            self.x_kubernetes_int_or_string = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_int_or_string,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_int_or_string,
                other.x_kubernetes_int_or_string,
            )?;
        }
        if self.x_kubernetes_list_map_keys.is_none() {
            self.x_kubernetes_list_map_keys = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_list_map_keys,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_list_map_keys,
                other.x_kubernetes_list_map_keys,
            )?;
        }
        if self.x_kubernetes_list_type.is_none() {
            self.x_kubernetes_list_type = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_list_type,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_list_type,
                other.x_kubernetes_list_type,
            )?;
        }
        if self.x_kubernetes_map_type.is_none() {
            self.x_kubernetes_map_type = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_map_type,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_map_type,
                other.x_kubernetes_map_type,
            )?;
        }
        if self.x_kubernetes_preserve_unknown_fields.is_none() {
            self.x_kubernetes_preserve_unknown_fields = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_preserve_unknown_fields,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_preserve_unknown_fields,
                other.x_kubernetes_preserve_unknown_fields,
            )?;
        }
        if self.x_kubernetes_validations.is_none() {
            self.x_kubernetes_validations = crate::OptionableConvert::try_from_optioned(
                other.x_kubernetes_validations,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.x_kubernetes_validations,
                other.x_kubernetes_validations,
            )?;
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
