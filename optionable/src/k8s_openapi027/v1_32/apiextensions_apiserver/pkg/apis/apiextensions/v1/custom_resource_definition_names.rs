#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceDefinitionNamesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plural: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_names: Option<std::vec::Vec<std::string::String>>,
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
        self.categories = other.categories;
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        self.list_kind = other.list_kind;
        if let Some(other_value) = other.plural {
            self.plural = other_value;
        }
        self.short_names = other.short_names;
        self.singular = other.singular;
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
