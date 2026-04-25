#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WebhookConversion describes how to call a conversion webhook
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebhookConversionAc {
    /// clientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_config: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookClientConfig as crate::Optionable>::Optioned,
    >,
    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_review_versions: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion {
    type Optioned = WebhookConversionAc;
}
#[automatically_derived]
impl crate::Optionable for WebhookConversionAc {
    type Optioned = WebhookConversionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion {
    fn into_optioned(self) -> WebhookConversionAc {
        WebhookConversionAc {
            client_config: crate::OptionableConvert::into_optioned(self.client_config),
            conversion_review_versions: Some(self.conversion_review_versions),
        }
    }
    fn try_from_optioned(value: WebhookConversionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            client_config: crate::OptionableConvert::try_from_optioned(
                value.client_config,
            )?,
            conversion_review_versions: value
                .conversion_review_versions
                .ok_or(crate::Error {
                    missing_field: "conversion_review_versions",
                })?,
        })
    }
    fn merge(&mut self, other: WebhookConversionAc) -> Result<(), crate::Error> {
        if self.client_config.is_none() {
            self.client_config = crate::OptionableConvert::try_from_optioned(
                other.client_config,
            )?;
        } else if let Some(self_value) = self.client_config.as_mut()
            && let Some(other_value) = other.client_config
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.conversion_review_versions {
            self.conversion_review_versions = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
> for WebhookConversionAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for WebhookConversionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.client_config,
            other.client_config,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.conversion_review_versions,
            other.conversion_review_versions,
        );
    }
}
