pub struct ExternalDocumentationAc {
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
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
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ExternalDocumentation {
    fn into_optioned(self) -> ExternalDocumentationAc {
        ExternalDocumentationAc {
            description: crate::OptionableConvert::into_optioned(self.description),
            url: crate::OptionableConvert::into_optioned(self.url),
        }
    }
    fn try_from_optioned(
        value: ExternalDocumentationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            description: crate::OptionableConvert::try_from_optioned(value.description)?,
            url: crate::OptionableConvert::try_from_optioned(value.url)?,
        })
    }
    fn merge(
        &mut self,
        other: ExternalDocumentationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.description, other.description)?;
        crate::OptionableConvert::merge(&mut self.url, other.url)?;
        Ok(())
    }
}
