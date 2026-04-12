#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalDocumentationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
    type Optioned = ExternalDocumentationAc;
}
#[automatically_derived]
impl crate::Optionable for ExternalDocumentationAc {
    type Optioned = ExternalDocumentationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
    fn into_optioned(self) -> ExternalDocumentationAc {
        ExternalDocumentationAc {
            description: self.description,
            url: self.url,
        }
    }
    fn try_from_optioned(value: ExternalDocumentationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            description: value.description,
            url: value.url,
        })
    }
    fn merge(&mut self, other: ExternalDocumentationAc) -> Result<(), crate::Error> {
        if other.description.is_some() {
            self.description = other.description;
        }
        if other.url.is_some() {
            self.url = other.url;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
> for ExternalDocumentationAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
