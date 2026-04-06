#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceConversionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<std::string::String>,
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
            self.strategy = other_value;
        }
        crate::OptionableConvert::merge(&mut self.webhook, other.webhook)?;
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
