#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceDefinitionNamesAc {
    /// categories is a list of grouped resources this custom resource belongs to (e.g. 'all'). This is published in API discovery documents, and used by clients to support invocations like `kubectl get all`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<std::vec::Vec<std::string::String>>,
    /// kind is the serialized kind of the resource. It is normally CamelCase and singular. Custom resource instances will use this value as the `kind` attribute in API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kind: Option<std::string::String>,
    /// plural is the plural name of the resource to serve. The custom resources are served under `/apis/\<group\>/\<version\>/.../\<plural\>`. Must match the name of the CustomResourceDefinition (in the form `\<names.plural\>.\<group\>`). Must be all lowercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plural: Option<std::string::String>,
    /// shortNames are short names for the resource, exposed in API discovery documents, and used by clients to support invocations like `kubectl get \<shortname\>`. It must be all lowercase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_names: Option<std::vec::Vec<std::string::String>>,
    /// singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames {
    type Optioned = CustomResourceDefinitionNamesAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionNamesAc {
    type Optioned = CustomResourceDefinitionNamesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames {
    fn into_optioned(self) -> CustomResourceDefinitionNamesAc {
        CustomResourceDefinitionNamesAc {
            categories: self.categories,
            kind: Some(self.kind),
            list_kind: self.list_kind,
            plural: Some(self.plural),
            short_names: self.short_names,
            singular: self.singular,
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionNamesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            categories: value.categories,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            list_kind: value.list_kind,
            plural: value
                .plural
                .ok_or(crate::Error {
                    missing_field: "plural",
                })?,
            short_names: value.short_names,
            singular: value.singular,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionNamesAc,
    ) -> Result<(), crate::Error> {
        if self.categories.is_none() {
            self.categories = crate::OptionableConvert::try_from_optioned(
                other.categories,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.categories, other.categories)?;
        }
        if let Some(other_value) = other.kind {
            self.kind = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.list_kind.is_none() {
            self.list_kind = crate::OptionableConvert::try_from_optioned(
                other.list_kind,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.list_kind, other.list_kind)?;
        }
        if let Some(other_value) = other.plural {
            self.plural = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.short_names.is_none() {
            self.short_names = crate::OptionableConvert::try_from_optioned(
                other.short_names,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.short_names, other.short_names)?;
        }
        if self.singular.is_none() {
            self.singular = crate::OptionableConvert::try_from_optioned(other.singular)?;
        } else {
            crate::OptionableConvert::merge(&mut self.singular, other.singular)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
> for CustomResourceDefinitionNamesAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
