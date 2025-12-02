#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct SelectableFieldAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    type Optioned = SelectableFieldAc;
}
#[automatically_derived]
impl crate::Optionable for SelectableFieldAc {
    type Optioned = SelectableFieldAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField {
    fn into_optioned(self) -> SelectableFieldAc {
        SelectableFieldAc {
            json_path: Some(crate::OptionableConvert::into_optioned(self.json_path)),
        }
    }
    fn try_from_optioned(value: SelectableFieldAc) -> Result<Self, crate::Error> {
        Ok(Self {
            json_path: crate::OptionableConvert::try_from_optioned(
                value
                    .json_path
                    .ok_or(crate::Error {
                        missing_field: "json_path",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SelectableFieldAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.json_path {
            crate::OptionableConvert::merge(&mut self.json_path, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
> for SelectableFieldAc {
    fn from_optionable(
        value: ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
