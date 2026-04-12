#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceConversion describes how to convert different versions of a CR.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceConversionAc {
    /// strategy specifies how custom resources are converted between versions. Allowed values are: - `"None"`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `"Webhook"`: API Server will call to an external webhook to do the conversion. Additional information
    ///   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<std::string::String>,
    /// webhook describes how to call the conversion webhook. Required when `strategy` is set to `"Webhook"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    type Optioned = CustomResourceConversionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceConversionAc {
    type Optioned = CustomResourceConversionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    fn into_optioned(self) -> CustomResourceConversionAc {
        CustomResourceConversionAc {
            strategy: Some(self.strategy),
            webhook: crate::OptionableConvert::into_optioned(self.webhook),
        }
    }
    fn try_from_optioned(
        value: CustomResourceConversionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            strategy: value
                .strategy
                .ok_or(crate::Error {
                    missing_field: "strategy",
                })?,
            webhook: crate::OptionableConvert::try_from_optioned(value.webhook)?,
        })
    }
    fn merge(&mut self, other: CustomResourceConversionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.strategy {
            self.strategy = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.webhook.is_none() {
            self.webhook = crate::OptionableConvert::try_from_optioned(other.webhook)?;
        } else if let Some(self_value) = self.webhook.as_mut()
            && let Some(other_value) = other.webhook
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
> for CustomResourceConversionAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
