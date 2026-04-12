#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SelectableField specifies the JSON path of a field that may be used with field selectors.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelectableFieldAc {
    /// jsonPath is a simple JSON path which is evaluated against each custom resource to produce a field selector value. Only JSON paths without the array notation are allowed. Must point to a field of type string, boolean or integer. Types with enum values and strings with formats are allowed. If jsonPath refers to absent field in a resource, the jsonPath evaluates to an empty string. Must not point to metdata fields. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    type Optioned = SelectableFieldAc;
}
#[automatically_derived]
impl crate::Optionable for SelectableFieldAc {
    type Optioned = SelectableFieldAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    fn into_optioned(self) -> SelectableFieldAc {
        SelectableFieldAc {
            json_path: Some(self.json_path),
        }
    }
    fn try_from_optioned(value: SelectableFieldAc) -> Result<Self, crate::Error> {
        Ok(Self {
            json_path: value
                .json_path
                .ok_or(crate::Error {
                    missing_field: "json_path",
                })?,
        })
    }
    fn merge(&mut self, other: SelectableFieldAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.json_path {
            self.json_path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
> for SelectableFieldAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
