#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
    type Optioned = ExternalDocumentationAc;
}
#[automatically_derived]
impl crate::Optionable for ExternalDocumentationAc {
    type Optioned = ExternalDocumentationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
    fn into_optioned(self) -> ExternalDocumentationAc {
        ExternalDocumentationAc {
            description: crate::OptionableConvert::into_optioned(self.description),
            url: crate::OptionableConvert::into_optioned(self.url),
        }
    }
    fn try_from_optioned(value: ExternalDocumentationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            description: crate::OptionableConvert::try_from_optioned(value.description)?,
            url: crate::OptionableConvert::try_from_optioned(value.url)?,
        })
    }
    fn merge(&mut self, other: ExternalDocumentationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.description, other.description)?;
        crate::OptionableConvert::merge(&mut self.url, other.url)?;
        Ok(())
    }
}
