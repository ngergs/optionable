pub struct ExternalDocumentationOpt {
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub url: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::external_documentation::ExternalDocumentation {
    type Optioned = ExternalDocumentationOpt;
}
#[automatically_derived]
impl crate::Optionable for ExternalDocumentationOpt {
    type Optioned = ExternalDocumentationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::external_documentation::ExternalDocumentation {
    fn into_optioned(self) -> ExternalDocumentationOpt {
        ExternalDocumentationOpt {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.description),
            url: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.url),
        }
    }
    fn try_from_optioned(
        value: ExternalDocumentationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            description: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.description)?,
            url: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.url)?,
        })
    }
    fn merge(
        &mut self,
        other: ExternalDocumentationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.description, other.description)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.url, other.url)?;
        Ok(())
    }
}
