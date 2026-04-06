#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceDefinitionVersion describes a version for CRD.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceDefinitionVersionAc {
    /// additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_printer_columns: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition as crate::Optionable>::Optioned,
        >,
    >,
    /// deprecated indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// deprecationWarning overrides the default warning returned to API clients. May only be set when `deprecated` is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: Option<std::string::String>,
    /// name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/\<group\>/\<version\>/...` if `served` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// schema describes the schema used for validation, pruning, and defaulting of this version of the custom resource.
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation as crate::Optionable>::Optioned,
    >,
    /// selectableFields specifies paths to fields that may be used as field selectors. A maximum of 8 selectable fields are allowed. See https://kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_fields: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField as crate::Optionable>::Optioned,
        >,
    >,
    /// served is a flag enabling/disabling this version from being served via REST APIs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served: Option<bool>,
    /// storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<bool>,
    /// subresources specify what subresources this version of the defined custom resource have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresources: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion {
    type Optioned = CustomResourceDefinitionVersionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionVersionAc {
    type Optioned = CustomResourceDefinitionVersionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion {
    fn into_optioned(self) -> CustomResourceDefinitionVersionAc {
        CustomResourceDefinitionVersionAc {
            additional_printer_columns: crate::OptionableConvert::into_optioned(
                self.additional_printer_columns,
            ),
            deprecated: self.deprecated,
            deprecation_warning: self.deprecation_warning,
            name: Some(self.name),
            schema: crate::OptionableConvert::into_optioned(self.schema),
            selectable_fields: crate::OptionableConvert::into_optioned(
                self.selectable_fields,
            ),
            served: Some(self.served),
            storage: Some(self.storage),
            subresources: crate::OptionableConvert::into_optioned(self.subresources),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionVersionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            additional_printer_columns: crate::OptionableConvert::try_from_optioned(
                value.additional_printer_columns,
            )?,
            deprecated: value.deprecated,
            deprecation_warning: value.deprecation_warning,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            schema: crate::OptionableConvert::try_from_optioned(value.schema)?,
            selectable_fields: crate::OptionableConvert::try_from_optioned(
                value.selectable_fields,
            )?,
            served: value
                .served
                .ok_or(crate::Error {
                    missing_field: "served",
                })?,
            storage: value
                .storage
                .ok_or(crate::Error {
                    missing_field: "storage",
                })?,
            subresources: crate::OptionableConvert::try_from_optioned(
                value.subresources,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionVersionAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.additional_printer_columns,
            other.additional_printer_columns,
        )?;
        self.deprecated = other.deprecated;
        self.deprecation_warning = other.deprecation_warning;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.schema, other.schema)?;
        crate::OptionableConvert::merge(
            &mut self.selectable_fields,
            other.selectable_fields,
        )?;
        if let Some(other_value) = other.served {
            self.served = other_value;
        }
        if let Some(other_value) = other.storage {
            self.storage = other_value;
        }
        crate::OptionableConvert::merge(&mut self.subresources, other.subresources)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
> for CustomResourceDefinitionVersionAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
