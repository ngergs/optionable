#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceDefinitionSpecAc {
    /// conversion defines conversion settings for the CRD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion as crate::Optionable>::Optioned,
    >,
    /// group is the API group of the defined custom resource. The custom resources are served under `/apis/\<group\>/...`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// names specify the resource and kind names for the custom resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::Optionable>::Optioned,
    >,
    /// preserveUnknownFields indicates that object fields which are not specified in the OpenAPI schema should be preserved when persisting to storage. apiVersion, kind, metadata and known fields inside metadata are always preserved. This field is deprecated in favor of setting `x-preserve-unknown-fields` to true in `spec.versions\[*\].schema.openAPIV3Schema`. See https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#field-pruning for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_unknown_fields: Option<bool>,
    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped. Allowed values are `Cluster` and `Namespaced`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<std::string::String>,
    /// versions is the list of all API versions of the defined custom resource. Version names are used to compute the order in which served versions are listed in API discovery. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    type Optioned = CustomResourceDefinitionSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionSpecAc {
    type Optioned = CustomResourceDefinitionSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    fn into_optioned(self) -> CustomResourceDefinitionSpecAc {
        CustomResourceDefinitionSpecAc {
            conversion: crate::OptionableConvert::into_optioned(self.conversion),
            group: Some(self.group),
            names: Some(crate::OptionableConvert::into_optioned(self.names)),
            preserve_unknown_fields: self.preserve_unknown_fields,
            scope: Some(self.scope),
            versions: Some(crate::OptionableConvert::into_optioned(self.versions)),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conversion: crate::OptionableConvert::try_from_optioned(value.conversion)?,
            group: value
                .group
                .ok_or(crate::Error {
                    missing_field: "group",
                })?,
            names: crate::OptionableConvert::try_from_optioned(
                value
                    .names
                    .ok_or(crate::Error {
                        missing_field: "names",
                    })?,
            )?,
            preserve_unknown_fields: value.preserve_unknown_fields,
            scope: value
                .scope
                .ok_or(crate::Error {
                    missing_field: "scope",
                })?,
            versions: crate::OptionableConvert::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionSpecAc,
    ) -> Result<(), crate::Error> {
        if self.conversion.is_none() {
            self.conversion = other.conversion;
        }
        if let Some(other_value) = other.conversion {
            crate::OptionableConvert::merge(&mut self.conversion, other_value)?;
        }
        if let Some(other_value) = other.group {
            self.group = other_value;
        }
        if let Some(other_value) = other.names {
            self.names = other_value;
        }
        if self.preserve_unknown_fields.is_none() {
            self.preserve_unknown_fields = other.preserve_unknown_fields;
        }
        if let Some(other_value) = other.preserve_unknown_fields {
            crate::OptionableConvert::merge(
                &mut self.preserve_unknown_fields,
                other_value,
            )?;
        }
        if let Some(other_value) = other.scope {
            self.scope = other_value;
        }
        if let Some(other_value) = other.versions {
            self.versions = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
> for CustomResourceDefinitionSpecAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
