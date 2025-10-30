pub struct WebhookConversionOpt {
    pub client_config: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
    > as crate::Optionable>::Optioned,
    pub conversion_review_versions: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion {
    type Optioned = WebhookConversionOpt;
}
#[automatically_derived]
impl crate::Optionable for WebhookConversionOpt {
    type Optioned = WebhookConversionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion {
    fn into_optioned(self) -> WebhookConversionOpt {
        WebhookConversionOpt {
            client_config: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
            > as crate::OptionableConvert>::into_optioned(self.client_config),
            conversion_review_versions: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(
                    self.conversion_review_versions,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: WebhookConversionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            client_config: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
            > as crate::OptionableConvert>::try_from_optioned(value.client_config)?,
            conversion_review_versions: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .conversion_review_versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "conversion_review_versions",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: WebhookConversionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig,
        > as crate::OptionableConvert>::merge(
            &mut self.client_config,
            other.client_config,
        )?;
        if let Some(other_value) = other.conversion_review_versions {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(
                &mut self.conversion_review_versions,
                other_value,
            )?;
        }
        Ok(())
    }
}
